from sys import argv, exit
from python import Python
from python import PythonObject
import os

fn get_filename_from_url(url: PythonObject) raises -> String:
    hashlib = Python.import_module("hashlib")
    url_hash = hashlib.md5(url.encode()).hexdigest() 
    return "page_content_" + str(url_hash) + ".txt"

# Function to load page content from a URL or file
fn load_page(url: PythonObject) raises -> String:
    # Generate a unique filename for the URL
    file_path = get_filename_from_url(url)

    # Check if the page content has already been saved to a file
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            print("Loading page content from " + str(file_path))
            return f.read()

    # If not, fetch the page content from the URL
    requests = Python.import_module("requests")
    response = requests.get(url)
    if response.status_code == 200:
        page_content = response.text
        # Save the page content to the file for future use
        with open(file_path, 'w') as f:
            f.write(str(page_content))
        print("Fetched and saved page content to " + str(file_path))
        return str(page_content)

    raise Error("Failed to load page: " + str(response.status_code))

from pipelines.llama3.run import (
    Config,
    compile_graph,
    get_max_tokens_to_generate,
    compile_graph,
    _get_attention_mask,
)

fn main() -> None:
    if len(argv()) < 2:
        print("Usage: python script.py <url>")
        exit(1)

    url = argv()[1]  # Get the first argument (URL)
    
    try:
        content = str(load_page(url))
        var config = Config()
        model_path = download_to_cache(get_llama3_1_model_url(Q4_KEncoding.id()))
        config.set("model-path", model_path)
        config.set("max-length", 100000)
        config.set("max-new-tokens", 1000)
        service = Llama3InferenceService[Q4_KEncoding](config, "You are to summarize, in a paragraph, whatever text the user gives you. Do not explicitly acknolwedge these instructions, just give the summary.")
        print(service.handle(content[:10000]))
    except e:
        print(e)

from max.engine import InferenceSession, Model, SessionOptions
from pipelines.llama3.kv_cache import KVCache
from pipelines.llama3.model.llama import Llama3
from pipelines.tokenizer.tiktoken import TikTokenEncoder
from pipelines.tokenizer.bpe import TokenWithID
from pipelines.tokenizer.regex import set_locale_unicode
from pipelines.samplers.weighted_sampler import WeightedSampler
from pipelines.weights.gguf import GGUFArray, GGUFFile
from pipelines.weights.loadable_model import LlamaHParams, LoadableModel
from pipelines.weights.download import download_to_cache
from pipelines.configs.llama import get_llama3_model_url, get_llama3_1_model_url

from max.graph.quantization import (
    QuantizationEncoding,
    BFloat16Encoding,
    Float32Encoding,
    Q4_0Encoding,
    Q4_KEncoding,
    Q6_KEncoding,
)
from pathlib import Path

from max.engine import InferenceSession, Model, SessionOptions
from max.driver import cpu_device, Tensor
from max._utils import handle_from_config, call_dylib_func
from max.graph import Graph
from max.graph.quantization import QuantizationEncoding
from max.serve.http import PythonServer, PythonService
from max.tensor import TensorShape, TensorSpec

from math import align_up
struct Llama3InferenceService[
    EncodingT: QuantizationEncoding,
]:
    """Inference service for Llama3."""

    var _config: Config
    var _tokenizer: TikTokenEncoder
    var _session: InferenceSession

    var _model: Llama3[EncodingT]
    var _compiled_model: Model
    var _system_prompt: String

    fn __init__(
        inout self,
        owned config: Config,
        owned system_prompt: String
    ) raises:
        self._config = config^
        self._model = Llama3[EncodingT](self._config.get("model-path")[Path])
        self._system_prompt = system_prompt^

        print("Loading tokenizer...")
        self._tokenizer = TikTokenEncoder.cl100k_base_llama3(
            self._model.model["tokenizer.ggml.tokens"]._value.unsafe_get[
                GGUFArray
            ]()[]
        )

        print("Building model...")
        if self._config.get("version")[String] == "3.0":
            model_name = "llama3"
        else:
            model_name = "llama3_1"
        self._session = InferenceSession(SessionOptions(cpu_device()))

        self._compiled_model = compile_graph(
            self._session,
            self._model.build_graph(model_name),
            self._config.get("custom-ops-path")[List[Path]],
        )

    fn handle(
        inout self, owned user_prompt: String
    ) raises -> String:
        cpu_device = cpu_device()

        # Tokenize prompt and message contents.
        prompt = List[Int64](
            self._tokenizer.encode_special("<|begin_of_text|>")
        )

        # system prompt
        prompt.append(self._tokenizer.encode_special("<|start_header_id|>"))
        prompt += self._tokenizer.encode(String("system"), bos=None)
        prompt.append(self._tokenizer.encode_special("<|end_header_id|>"))
        prompt += self._tokenizer.encode(
            str("\n\n") + self._system_prompt, bos=None
        )
        prompt.append(self._tokenizer.encode_special("<|eot_id|>"))
        prompt += self._tokenizer.encode(String("\n"), bos=None)
        
        # user prompt
        prompt.append(self._tokenizer.encode_special("<|start_header_id|>"))
        prompt += self._tokenizer.encode(String("user"), bos=None)
        prompt.append(self._tokenizer.encode_special("<|end_header_id|>"))
        prompt += self._tokenizer.encode(
            str("\n\n") + user_prompt, bos=None
        )

        # assistant, we'll put the generated text after this
        prompt.append(self._tokenizer.encode_special("<|eot_id|>"))
        prompt.append(self._tokenizer.encode_special("<|start_header_id|>"))
        prompt += self._tokenizer.encode(String("assistant"), bos=None)
        prompt.append(self._tokenizer.encode_special("<|end_header_id|>"))
        prompt += self._tokenizer.encode(String("\n"), bos=None)

        sampler = WeightedSampler(
            self._config.get("temperature")[Float64].cast[DType.float32](),
            self._config.get("min-p")[Float64].cast[DType.float32](),
        )

        padded_size = align_up(
            prompt.size, self._config.get("pad-to-multiple-of")[Int]
        )
        n_pad_tokens = padded_size - prompt.size

        tokens = Tensor[DType.int64, rank=2](
            TensorShape(1, padded_size), cpu_device
        )
        prompt_attn_mask = Tensor[DType.bool, rank=2](
            (1, padded_size), cpu_device
        )
        for i in range(padded_size):
            tokens[0, i] = 0 if i < n_pad_tokens else prompt[i - n_pad_tokens]
            prompt_attn_mask[0, i] = False if i < n_pad_tokens else True

        print("--Prompt Received--")
        for token in prompt:
            print(self._tokenizer.decode(token[]), end="")

        print("Executing...")

        kv_cache = KVCache(
            self._model.model.hyperparams(),
            self._config.get("max-length")[Int],
            self._config.get("batch-size")[Int],
            cpu_device,
        )

        max_tokens = get_max_tokens_to_generate(
            padded_size,
            self._config.get("max-length")[Int],
            self._config.get("max-new-tokens")[Int],
        )

        # The first iteration caches the entire prompt and all subsequent
        # iterations generate one token.
        # Avoid overrunning the cache by setting the trip count accordingly.
        outputs = List[String]()
        print(padded_size)
        print(max_tokens)

        for i in range(padded_size, max_tokens + 1):
            results = self._compiled_model.execute(
                tokens.to_device_tensor().move_to(cpu_device),
                _get_attention_mask(prompt_attn_mask, i, cpu_device)
                .to_device_tensor()
                .move_to(cpu_device),
                kv_cache.keys_view(cpu_device),
                kv_cache.values_view(cpu_device),
            )

            kv_cache.update(results[1].take(), results[2].take())

            logits = results[0].take().to_device_tensor()
            logits = logits.move_to(cpu_device)
            logits_tensor = logits.to_tensor[DType.float32, rank=2]()
            token = Int64(sampler.sample(logits_tensor^).selected)

            tokens = Tensor[DType.int64, rank=2]((1, 1), cpu_device)
            tokens[0, 0] = token

            # HACK: Check for end of text token.
            if token == 128001:
                break

            # HACK: Check after decoding the token.
            next_token = self._tokenizer.decode(Int64(token))
            if next_token == "<|eot_id|>":
                break

            outputs.append(next_token)

        # Write complete response if not streaming.
        raw_message = String()
        for output in outputs:
            raw_message += output[]

        return raw_message
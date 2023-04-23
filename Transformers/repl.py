#!/usr/bin/env python3
import sys
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM, GenerationConfig

line = 'What color is the undoubtedly beautiful sky?'

model_name = 'google/flan-t5-xl'
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForSeq2SeqLM.from_pretrained(model_name)

config = GenerationConfig(max_new_tokens=200)
for line in sys.stdin:
    tokens = tokenizer(line, return_tensors="pt")
    outputs = model.generate(**tokens, generation_config=config)
    print(tokenizer.batch_decode(outputs, skip_special_tokens=True))

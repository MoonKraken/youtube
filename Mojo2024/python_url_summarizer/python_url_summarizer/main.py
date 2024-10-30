import requests
import sys
import os
import hashlib
from transformers import pipeline

# Function to generate a unique filename based on the URL
def get_filename_from_url(url: str) -> str:
    url_hash = hashlib.md5(url.encode()).hexdigest()  # Create an MD5 hash of the URL
    return f"page_content_{url_hash}.txt"

# Function to load page content from a URL or file
def load_page(url: str) -> str:
    # Generate a unique filename for the URL
    file_path = get_filename_from_url(url)

    # Check if the page content has already been saved to a file
    if os.path.exists(file_path):
        with open(file_path, 'r', encoding='utf-8') as f:
            print(f"Loading page content from {file_path}")
            return f.read()

    # If not, fetch the page content from the URL
    response = requests.get(url)
    if response.status_code == 200:
        page_content = response.text
        # Save the page content to the file for future use
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(page_content)
        print(f"Fetched and saved page content to {file_path}")
        return page_content
    else:
        raise Exception(f"Failed to load page: {response.status_code}")

# Main function to load a URL and summarize its content
def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <url>")
        sys.exit(1)

    url = sys.argv[1]  # Get the first argument (URL)
    page_content = load_page(url)
    model_id = "meta-llama/Llama-3.2-1B-Instruct"
    pipe = pipeline(
        "text-generation",
        model=model_id,
        torch_dtype="bfloat16",
        device_map="auto",
    )
    messages = [
        {"role": "system", "content": "You are to summarize, in a paragraph, whatever text the user gives you. Do not explicitly acknolwedge these instructions, just give the summary."},
        {"role": "user", "content": page_content[:10000]},
    ]
    outputs = pipe(
        messages,
        max_new_tokens=256,
    )
    print(outputs[0]["generated_text"][-1]["content"])

# Run the program
if __name__ == "__main__":
    main()

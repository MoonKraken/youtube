#!/usr/bin/env python3

# run this in the same session as nuclia_upload.py
# so we can use my_kb and encoder that are defined there
vectorset_name = "all-MiniLM-L6-v2"
search_str = "Brains have axons and dendrites."
query_vectors = encoder.encode([search_str])
results = my_kb.search(
    vector = query_vectors[0],
    filter = ['programming/things'],
    vectorset=vectorset_name
)

for result in results:
    print(f"Text: {result.text}")
    print(f"Labels: {result.labels}")
    print(f"Score: {result.score}")
    print(f"Key: {result.key}")
    print(f"Score Type: {result.score_type}")

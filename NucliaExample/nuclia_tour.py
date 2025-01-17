#!/usr/bin/env python3

from nucliadb_sdk.client import Environment, NucliaDBClient
from nucliadb_sdk import KnowledgeBox

from sentence_transformers import SentenceTransformer
encoder = SentenceTransformer("all-MiniLM-L6-v2")
documents = [
    "In 2023, it appears that we are on the cusp of artificial general intelligence",
    "Neural networks are getting really smart in recent days!",
    "Rust is a really nice programming language!",
    "The sky is blue."
]

from nucliadb_sdk import create_knowledge_box, delete_kb
#delete_kb("my_new_kb")
my_kb = create_knowledge_box("my_new_kb")

vectorset_name = "all-MiniLM-L6-v2"

for i in range(0, len(documents)):
    document = documents[i]
    vectors = encoder.encode([document])
    resource_id = my_kb.upload(
        key='mykey' + str(i),
        text=document,
        labels=['programming/things'],
        vectors={vectorset_name: vectors[0]},
    )

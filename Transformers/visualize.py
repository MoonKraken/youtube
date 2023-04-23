#!/usr/bin/env python3

import torch
import matplotlib.pyplot as plt
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM
from sklearn.decomposition import PCA
import numpy as np
from mpl_toolkits.mplot3d import Axes3D

# Load the tokenizer and the model

model_name = 'google/flan-t5-base'
model = AutoModelForSeq2SeqLM.from_pretrained(model_name)
tokenizer = AutoTokenizer.from_pretrained(model_name)

# Define the tokens you want to visualize
tokens = [
    "house",
    "home",
    "sky",
    "air",
    "atmosphere",
    "crab",
    "lobster",
    "shrimp",
    "computer",
    "machine",
    "electronic",
    "wind",
    "dwelling",
    "office",
    "village",
    "hearth",
    "cat",
    "fire",
    "flame",
    "blaze",
    "spark",
    "dog",
    "animal",
    "structure",
    "building"
]

# Convert tokens to IDs
input_ids = tokenizer.convert_tokens_to_ids(tokens)

# Get the input embeddings from the model
input_embeddings = model.get_input_embeddings()

# Retrieve the embedding vectors for the tokens
token_embeddings = [input_embeddings(torch.tensor([token_id])) for token_id in input_ids]

# Convert the tensors to NumPy format
token_embeddings_np = [embedding.detach().numpy() for embedding in token_embeddings]

# Stack the token embeddings
token_embeddings_np = np.vstack(token_embeddings_np)

# Apply PCA to reduce the dimensionality of the embedding vectors to 3D
pca = PCA(n_components=3)
token_embeddings_3d = pca.fit_transform(token_embeddings_np)

# Create a 3D scatter plot
fig = plt.figure()
ax = fig.add_subplot(111, projection="3d")
ax.scatter(token_embeddings_3d[:, 0], token_embeddings_3d[:, 1], token_embeddings_3d[:, 2])

# Annotate the plot with the tokens
for i, token in enumerate(tokens):
    ax.text(token_embeddings_3d[i, 0], token_embeddings_3d[i, 1], token_embeddings_3d[i, 2], token)

ax.set_xlabel("PCA 1")
ax.set_ylabel("PCA 2")
ax.set_zlabel("PCA 3")
plt.title("3D representation of token embeddings")
plt.show()

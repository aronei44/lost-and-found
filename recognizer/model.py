import io
from PIL import Image
import numpy as np
import face_recognition
import faiss

def resize_image(image_bytes, max_size=800):
    img = Image.open(io.BytesIO(image_bytes))
    img.thumbnail((max_size, max_size))
    buf = io.BytesIO()
    img.save(buf, format='JPEG')
    return buf.getvalue()

def get_face_embeddings(image_bytes):
    resized = resize_image(image_bytes)
    image = face_recognition.load_image_file(io.BytesIO(resized))
    encodings = face_recognition.face_encodings(image)
    return encodings

def build_faiss_index(db):
    """
    db = dict {name: [list embedding]}
    Return: faiss index, list nama per embedding index
    """
    embeddings = []
    labels = []
    for name, embs in db.items():
        embeddings.extend(embs)
        labels.extend([name]*len(embs))

    dim = 128  # fixed embedding size for face_recognition
    index = faiss.IndexFlatL2(dim)
    if embeddings:
        embeddings_np = np.array(embeddings).astype('float32')
        index.add(embeddings_np)
    return index, labels

def search_faiss(index, labels, query_embedding, threshold=0.6):
    query = np.array([query_embedding]).astype('float32')
    D, I = index.search(query, k=1)
    dist = D[0][0]
    idx = I[0][0]

    # Threshold pakai squared distance
    if dist < threshold**2:
        return labels[idx], dist
    else:
        return "Unknown", dist
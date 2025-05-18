import pickle
import os

DB_FILE = "face_db.pkl"

def load_db():
    if os.path.exists(DB_FILE):
        with open(DB_FILE, "rb") as f:
            return pickle.load(f)
    return {}

def save_db(db):
    with open(DB_FILE, "wb") as f:
        pickle.dump(db, f)

def add_embeddings(name, embeddings):
    db = load_db()
    db.setdefault(name, []).extend(embeddings)
    save_db(db)

def get_db():
    return load_db()
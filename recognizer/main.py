from fastapi import FastAPI, UploadFile, File, Form, HTTPException
from fastapi.responses import JSONResponse
from typing import List
from datetime import datetime
import os
import io

from model import get_face_embeddings, build_faiss_index, search_faiss
from storage import add_embeddings, get_db

app = FastAPI()
STORAGE_DIR = "storage"
os.makedirs(STORAGE_DIR, exist_ok=True)

@app.post("/add_target")
async def add_target(name: str = Form(...), files: List[UploadFile] = File(...)):
    if not files:
        raise HTTPException(status_code=400, detail="Tidak ada file yang diupload.")

    embeddings = []
    for file in files:
        try:
            contents = await file.read()
            encodings = get_face_embeddings(contents)
            if not encodings:
                continue
            embeddings.append(encodings[0])

            # Simpan file di storage (optional)
            save_dir = os.path.join(STORAGE_DIR, name)
            os.makedirs(save_dir, exist_ok=True)
            timestamp = datetime.now().strftime("%Y-%m-%d_%H-%M-%S-%f")
            ext = os.path.splitext(file.filename)[-1].lower()
            save_path = os.path.join(save_dir, f"{timestamp}{ext}")
            with open(save_path, "wb") as f:
                f.write(contents)
        except Exception as e:
            pass

    if not embeddings:
        raise HTTPException(status_code=400, detail="Tidak ada wajah terdeteksi di semua foto.")

    add_embeddings(name, embeddings)

    return {"message": f"Target '{name}' berhasil ditambahkan dengan {len(embeddings)} foto."}

@app.post("/recognize")
async def recognize(file: UploadFile = File(...)):
    try:
        contents = await file.read()
        encodings = get_face_embeddings(contents)
        if not encodings:
            return JSONResponse(status_code=400, content={"message": "Tidak ada wajah terdeteksi di gambar."})

        db = get_db()
        index, labels = build_faiss_index(db)

        recognized_names = []
        for embedding in encodings:
            name, _ = search_faiss(index, labels, embedding)
            recognized_names.append(name)

        return {"recognized": recognized_names}
    except Exception as e:
        return JSONResponse(status_code=500, content={"message": str(e)})
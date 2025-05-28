from fastapi import FastAPI, UploadFile, File, Form, HTTPException
from fastapi.responses import JSONResponse
from typing import List
from datetime import datetime
import os
from minio import Minio
from minio.error import S3Error
import dotenv
from io import BytesIO
dotenv.load_dotenv()

from model import get_face_embeddings, build_faiss_index, search_faiss
from storage import add_embeddings, get_db

app = FastAPI(root_path="/api/recognition")
STORAGE_DIR = "storage"
os.makedirs(STORAGE_DIR, exist_ok=True)

MINIO_ENDPOINT = os.getenv("MINIO_ENDPOINT", "localhost:9000")
MINIO_ACCESS_KEY = os.getenv("MINIO_ACCESS_KEY", "minioadmin")
MINIO_SECRET_KEY = os.getenv("MINIO_SECRET_KEY", "minioadmin")
BUCKET_NAME = os.getenv("MINIO_BUCKET", "face-recognition")
MINIO_USE_SSL = os.getenv("MINIO_USE_SSL", "false").lower() == "true"

# Inisialisasi MinIO client
minio_client = Minio(
    MINIO_ENDPOINT,
    access_key=MINIO_ACCESS_KEY,
    secret_key=MINIO_SECRET_KEY,
    secure=MINIO_USE_SSL,
)

@app.post("/add_target")
async def add_target(name: str = Form(...), files: List[UploadFile] = File(...)):
    if not files:
        raise HTTPException(status_code=400, detail="Tidak ada file yang diupload.")

    embeddings = []
    saved_files = []

    for file in files:
        try:
            contents = await file.read()
            encodings = get_face_embeddings(contents)
            if not encodings:
                continue
            embeddings.append(encodings[0])

            # Simpan ke MinIO
            timestamp = datetime.now().strftime("%Y-%m-%d_%H-%M-%S-%f")
            ext = os.path.splitext(file.filename)[-1].lower()
            object_name = f"{name}/{timestamp}{ext}"

            # Upload to MinIO
            stream = BytesIO(contents)
            minio_client.put_object(
                BUCKET_NAME,
                object_name,
                data=stream,
                length=len(contents),
                content_type=file.content_type
            )
            saved_files.append(object_name)
            
        except S3Error as e:
            print(f"Gagal menyimpan ke MinIO: {e}")
            continue
        except Exception as e:
            print(f"Error processing file {file.filename}: {e}")
            continue

    if not embeddings:
        raise HTTPException(status_code=400, detail="Tidak ada wajah terdeteksi di semua foto.")

    add_embeddings(name, embeddings)

    return {"message": f"Target '{name}' berhasil ditambahkan dengan {len(embeddings)} foto.", "saved_files": saved_files}

@app.post("/recognize")
async def recognize(file: UploadFile = File(...)):
    try:
        contents = await file.read()
        encodings = get_face_embeddings(contents)
        if not encodings:
            return JSONResponse(status_code=400, content={"message": "Tidak ada wajah terdeteksi di gambar."})
        
        timestamp = datetime.now().strftime("%Y-%m-%d_%H-%M-%S-%f")
        ext = os.path.splitext(file.filename)[-1].lower()
        object_name = f"found_people/{timestamp}{ext}"

        # Upload to MinIO
        stream = BytesIO(contents)
        minio_client.put_object(
            BUCKET_NAME,
            object_name,
            data=stream,
            length=len(contents),
            content_type=file.content_type
        )

        db = get_db()
        index, labels = build_faiss_index(db)

        recognized_names = []
        for embedding in encodings:
            name, _ = search_faiss(index, labels, embedding)
            recognized_names.append(name)

        return {"recognized": recognized_names, "saved_file": object_name}
    except Exception as e:
        return JSONResponse(status_code=500, content={"message": str(e)})
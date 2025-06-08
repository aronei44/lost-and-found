"use client";
import Card from "./Card";
import Input from "./Input";
import { useState } from "react";
import { apiClient } from "@/hooks/useApi";
import Swal from "sweetalert2";
import { useGlobalContext } from "@/hooks/globalprovider";

const AddPhoto = (p: {
    id: number,
    setShowPhotoModal: (show: boolean) => void;
}) => {
    
    const {
        state: {
            token
        }
    } = useGlobalContext();
    const [files, setFiles] = useState<File[]>([]);

    const addPhoto = async () => {
        try {
            const formData = new FormData();
            files.forEach(file => {
                formData.append('files', file);
            });
            const resp = await apiClient({
                method: "post",
                url: `/api/data/upload_photo/${p.id}`,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`,
                    "Content-Type": "multipart/form-data"
                },
                data: formData
            })
            Swal.fire({
                title: "Add Photo Successful",
                icon: "success",
                text: resp,
                confirmButtonText: "OK"
            });
            p.setShowPhotoModal(false);
        } catch (error) {
            console.error("Add Photo Failed:", error);
            Swal.fire({
                title: "Add Photo Failed",
                icon: "error",
                confirmButtonText: "OK"
            });
        }
    }
    return (
        <div className="col-span-2">

            <Card>
                <h1 className="text-xl font-bold">Add Photo</h1>
                <hr className="mb-6" />
                <Input
                    id="fullname"
                    label="Full Name"
                    type="file"
                    required
                    placeholder="Enter full name"
                    onChange={(e) => setFiles(Array.from(e.target.files || []))}
                />
                <button
                    className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer"
                    onClick={() => {
                        addPhoto();
                    }}
                >
                    Upload
                </button>
            </Card>
        </div>
    )
}

export default AddPhoto;
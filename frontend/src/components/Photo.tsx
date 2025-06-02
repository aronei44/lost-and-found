"use client";
import Card from "./Card";
import { useEffect, useState } from "react";
import { apiClient } from "@/hooks/useApi";
import Swal from "sweetalert2";
import { useGlobalContext } from "@/hooks/globalprovider";
import Image from "next/image";
import { serverKey } from "@/hooks/useServerKey";

const Photo = (p: {
    id: number,
    place_id?: number,
}) => {
    
    const {
        state: {
            token
        }
    } = useGlobalContext();
    const [data, setData] = useState<Array<{
        id: number,
        bucket: string,
        path: string,
    }>>([]);
    const [baseUrl, setBaseUrl] = useState<string>("");

    const getDataPhoto = async () => {
        try {
            let url = `/api/data/photos/${p.id}`;
            if (p.place_id) {
                url = `/api/data/photos/${p.id}/${p.place_id}`;
            }   
            const response = await apiClient({
                method: "get",
                url,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                }
            })
            setData(response);
        } catch (error) {
            console.error("Get Photo Failed:", error);
            Swal.fire({
                title: "Get Photo Failed",
                icon: "error",
                confirmButtonText: "OK"
            });
        }
    }

    useEffect(() => {
        getDataPhoto();
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [p.id]);

    const getBaseUrl = async () => {
        const url = await serverKey("MINIO_URL");
        setBaseUrl(url);
    }

    useEffect(() => {
        getBaseUrl();
    }, []);

    return (
        <div className="col-span-2">
            <Card>
                <h1 className="text-xl font-bold">Photo</h1>
                <hr className="mb-6" />
                <div className="grid grid-cols-4 gap-4">
                    {data.map((photo) => (
                        <Image
                            key={photo.id}
                            src={`${baseUrl}/${photo.bucket}/${photo.path}`}
                            alt={`Photo ${photo.id}`}
                            width={200}
                            height={200}
                        />
                    ))}
                </div>
            </Card>
        </div>
    )
}

export default Photo;
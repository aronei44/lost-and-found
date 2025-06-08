"use client";
import Card from "./Card";
import { useEffect, useState } from "react";
import { apiClient } from "@/hooks/useApi";
import Swal from "sweetalert2";
import { useGlobalContext } from "@/hooks/globalprovider";
import Photo from "./Photo";
import ModalComponent from "./Modal";
import Profile from "./Profile";

const Trace = (p: {
    id: number,
}) => {
    
    const {
        state: {
            token
        }
    } = useGlobalContext();
    const [data, setData] = useState<Array<{
        id: number,
        latitude: string,
        longitude: string,
        user_username: string,
        created_at: string,
    }>>([]);
    const [showModal, setShowModal] = useState<boolean>(false);
    const [placeId, setPlaceId] = useState<number>(0);
    const [user, setUser] = useState<string>("");

    const getDataTrace = async () => {
        try {
            const response = await apiClient({
                method: "get",
                url: `/api/data/places/${p.id}`,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                }
            })
            setData(response);
        } catch (error) {
            console.error("Get Trace Failed:", error);
            Swal.fire({
                title: "Get Trace Failed",
                icon: "error",
                confirmButtonText: "OK"
            });
        }
    }

    useEffect(() => {
        getDataTrace();
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [p.id]);

    return (
        <Card>
            <h1 className="text-xl font-bold">Jejak</h1>
            <hr className="mb-6" />

            <div className="relative overflow-x-auto">
                <table className="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                    <thead className="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                        <tr>
                            <th scope="col" className="px-6 py-3">
                                Nama Penemu
                            </th>
                            <th scope="col" className="px-6 py-3">
                                Latitude
                            </th>
                            <th scope="col" className="px-6 py-3">
                                Longitude
                            </th>
                            <th scope="col" className="px-6 py-3">
                                Waktu Terdeteksi
                            </th>
                            <th scope="col" className="px-6 py-3">
                                Aksi
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {data.map((trace) => (
                            <tr key={trace.id} className="bg-white border-b dark:bg-gray-800 dark:border-gray-700 border-gray-200">
                                <td className="px-6 py-4">
                                    {trace.user_username}
                                </td>
                                <td className="px-6 py-4">
                                    {trace.latitude}
                                </td>
                                <td className="px-6 py-4">
                                    {trace.longitude}
                                </td>
                                <td className="px-6 py-4">
                                    {new Date(trace.created_at).toLocaleString()}
                                </td>
                                <td className="px-6 py-4">
                                    <a
                                        href={`https://www.google.com/maps/search/?api=1&query=${trace.latitude},${trace.longitude}`}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer mr-2"
                                    >
                                        Lihat di Peta
                                    </a>
                                    <button
                                        className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer"
                                        onClick={() => {
                                            setPlaceId(trace.id);
                                            setUser(trace.user_username);
                                            setShowModal(true);
                                        }}
                                    >
                                        Lihat Detail
                                    </button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
            <ModalComponent
                show={showModal}
                onClose={() => {
                    setShowModal(false);
                }}
                title="Detail Orang Hilang"
            >
                <div className="grid grid-cols-3 gap-4">
                    <Photo id={p.id} place_id={placeId} />
                    <Profile username={user} />
                </div>
            </ModalComponent>
        </Card>
    )
}

export default Trace;
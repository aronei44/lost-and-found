"use client";
import Card from "./Card";
import { useEffect, useState } from "react";
import { apiClient } from "@/hooks/useApi";
import { useGlobalContext } from "@/hooks/globalprovider";
import ModalComponent from "./Modal";
import Detail from "./Detail";
import Photo from "./Photo";
import AddPhoto from "./AddPhoto";
import AddLostPerson from "./AddLostPerson";

const LostPeople = (p: {
    all: boolean
}) => {

    const {
        state: {
            token
        }
    } = useGlobalContext();

    const [data, setData] = useState<Array<{
        fullname: string,
        alias: string,
        born_date: string,
        lost_date: string,
        last_condition: string,
        gender: string,
        id: number,
        is_found: boolean,
        found_date?: string
    }>>([]);
    const [showModal, setShowModal] = useState<boolean>(false);
    const [showPhotoModal, setShowPhotoModal] = useState<boolean>(false);
    const [showAddModal, setShowAddModal] = useState<boolean>(false);
    const [id, setId] = useState<number>(0);

    const getLostPeople = async () => {
        try {
            const response = await apiClient({
                method: "get",
                url: `/api/lost_people${p.all ? '/all' : ''}`,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                }
            });
            setData(response);
        } catch (error) {
            console.error("Failed to fetch lost people:", error);
        }
    }

    const addMonitoredLostPeople = async (id: string) => {
        try {
            await apiClient({
                method: "post",
                url: `/api/lost_people/${id}`,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                }
            });
        } catch (error) {
            console.error("Failed to add monitored lost people:", error);
        }
    }
    useEffect(() => {
        getLostPeople();
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [showAddModal]);

    return (
        <div className="container mx-auto p-4 mt-20">
            <div className="flex justify-between items-center mb-4">
                { !p.all && (
                    <button
                        className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer"
                        onClick={() => setShowAddModal(true)}
                    >
                        Tambah Orang Hilang
                    </button>
                )}
                { showAddModal && (
                    <ModalComponent
                        show={showAddModal}
                        onClose={() => setShowAddModal(false)}
                        title="Tambah Orang Hilang"
                    >
                        <AddLostPerson setShowAddModal={setShowAddModal} />
                    </ModalComponent>
                )}
            </div>
            <Card>
                <h1 className="text-6xl font-bold mt-10">Data Orang Dicari</h1>
                <hr className="mb-6"/>

                <div className="relative overflow-x-auto">
                    <table className="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                        <thead className="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                            <tr>
                                <th scope="col" className="px-6 py-3">
                                    Nama Lengkap
                                </th>
                                <th scope="col" className="px-6 py-3">
                                    Alias
                                </th>
                                <th scope="col" className="px-6 py-3">
                                    Jenis Kelamin
                                </th>
                                <th scope="col" className="px-6 py-3">
                                    Tanggal Lahir
                                </th>
                                <th scope="col" className="px-6 py-3">
                                    Tanggal Hilang
                                </th>
                                <th scope="col" className="px-6 py-3">
                                    Kondisi Terakhir
                                </th>
                                { !p.all && (
                                    <>
                                        <th scope="col" className="px-6 py-3">
                                            Status
                                        </th>
                                        <th scope="col" className="px-6 py-3">
                                            Tanggal Ditemukan
                                        </th>
                                    </>
                                )}
                                <th scope="col" className="px-6 py-3">
                                    Aksi
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {data.map((person) => (
                                <tr key={person.id} className="bg-white border-b dark:bg-gray-800 dark:border-gray-700 border-gray-200">
                                    <th scope="row" className="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                        {person.fullname}
                                    </th>
                                    <td className="px-6 py-4">
                                        {person.alias}
                                    </td>
                                    <td className="px-6 py-4">
                                        {person.gender === 'L' ? 'Laki-laki' : 'Perempuan'}
                                    </td>
                                    <td className="px-6 py-4">
                                        {new Date(person.born_date).toLocaleDateString()}
                                    </td>
                                    <td className="px-6 py-4">
                                        {new Date(person.lost_date).toLocaleDateString()}
                                    </td>
                                    <td className="px-6 py-4">
                                        {person.last_condition}
                                    </td>
                                    { !p.all && (
                                        <>
                                            <td className="px-6 py-4">
                                                {person.is_found ? 'Ditemukan' : 'Belum Ditemukan'}
                                            </td>
                                            <td className="px-6 py-4">
                                                {person.found_date ? new Date(person.found_date).toLocaleDateString() : '-'}
                                            </td>
                                        </>
                                    )}

                                    <td className="py-2">
                                        <button
                                            className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer"
                                            onClick={() => {
                                                setId(person.id);
                                                setShowModal(true);
                                            }}
                                        >
                                            Lihat Detail
                                        </button>
                                        { p.all ? (
                                            <button
                                                className="bg-green-500 text-white px-4 py-2 rounded cursor-pointer ml-4"
                                                onClick={() => {
                                                    addMonitoredLostPeople(person.id.toString());
                                                }}
                                            >
                                               + Monitor
                                            </button>
                                        ) : (
                                            <button
                                                className="bg-green-500 text-white px-4 py-2 rounded cursor-pointer ml-4 mt-4"
                                                onClick={() => {
                                                    setId(person.id);
                                                    setShowPhotoModal(true);
                                                }}
                                            >
                                                + Foto
                                            </button>
                                        )}
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </Card>
            <ModalComponent
                show={showModal}
                onClose={() => {
                    setShowModal(false);
                }}
                title="Detail Orang Hilang"
            >
                <div className="grid grid-cols-3 gap-4">
                    <Detail
                        id={id}
                        disabled={true}
                    />
                    <Photo id={id} />
                </div>
            </ModalComponent>
            <ModalComponent
                show={showPhotoModal}
                onClose={() => {
                    setShowPhotoModal(false);
                }}
                title="Foto Orang Hilang"
            >
                <div className="grid grid-cols-4 gap-4">
                    <Photo id={id} />
                    <AddPhoto id={id} setShowPhotoModal={setShowPhotoModal} />
                </div>
            </ModalComponent>
        </div>
    )
}

export default LostPeople;
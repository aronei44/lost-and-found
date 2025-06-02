"use client";
import Card from "./Card";
import Input from "./Input";
import { useEffect, useState } from "react";
import { apiClient } from "@/hooks/useApi";
import Swal from "sweetalert2";
import { useGlobalContext } from "@/hooks/globalprovider";
import ModalComponent from "./Modal";
import Trace from "./Trace";

const Detail = (p: {
    id: number,
    disabled?: boolean
}) => {
    
    const {
        state: {
            token
        }
    } = useGlobalContext();

    const [form, setForm] = useState<{
        fullname: string,
        alias: string,
        born_date: string,
        lost_date: string,
        last_condition: string,
        gender: string,
        id: number,
        is_found: boolean,
        found_date?: string
    }>({
        fullname: '',
        alias: '',
        born_date: '',
        lost_date: '',
        last_condition: '',
        gender: '',
        id: 0,
        is_found: false,
        found_date: ''
    })
    const [showModal, setShowModal] = useState<boolean>(false);

    const getDataPerson = async () => {
        try {
            if (p.id) {
                const response = await apiClient({
                    method: "get",
                    url: `/api/lost_people/${p.id}`,
                    headers: {
                        Authorization: `Bearer ${token.accessToken}`
                    }
                })
                setForm(response);
            }
        } catch (error) {
            console.error("Get Detail Failed:", error);
            Swal.fire({
                title: "Get Detail Failed",
                icon: "error",
                confirmButtonText: "OK"
            });
        }
    }

    useEffect(() => {
        getDataPerson();
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [p.id]);

    return (
        <Card>
            <h1 className="text-xl font-bold">Detail</h1>
            <hr className="mb-6" />
            <Input
                id="fullname"
                label="Full Name"
                type="text"
                required
                placeholder="Enter full name"
                onChange={(e) => setForm({ ...form, fullname: e.target.value })}
                value={form.fullname}
                disabled={p.disabled}
            />
            <Input
                id="alias"
                label="Alias"
                type="text"
                required
                placeholder="Enter alias"
                onChange={(e) => setForm({ ...form, alias: e.target.value })}
                value={form.alias}
                disabled={p.disabled}
            />
            <Input
                id="born_date"
                label="Born Date"
                type="date"
                required
                placeholder="Enter born date"
                onChange={(e) => setForm({ ...form, born_date: e.target.value })}
                value={form.born_date}
                disabled={p.disabled}
            />
            <Input
                id="lost_date"
                label="Lost Date"
                type="date"
                required
                placeholder="Enter lost date"
                onChange={(e) => setForm({ ...form, lost_date: e.target.value })}
                value={form.lost_date}
                disabled={p.disabled}
            />
            <Input
                id="last_condition"
                label="Last Condition"
                type="text"
                required
                placeholder="Enter last condition"
                onChange={(e) => setForm({ ...form, last_condition: e.target.value })}
                value={form.last_condition}
                disabled={p.disabled}
            />
            <div className="mb-4">
                <label htmlFor="gender" className="block text-gray-700 font-bold mb-2">
                    Gender
                </label>
                <select
                    id="gender"
                    value={form.gender}
                    onChange={(e) => setForm({ ...form, gender: e.target.value })}
                    disabled={p.disabled}
                    className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    required
                >
                    <option value="">Select Gender</option>
                    <option value="L">Laki Laki</option>
                    <option value="P">Perempuan</option>
                </select>
            </div>
            <Input
                id="found_date"
                label="Found Date"
                type="date"
                placeholder="Enter found date"
                onChange={(e) => setForm({ ...form, found_date: e.target.value })}
                value={form.found_date}
                disabled={true}
            />
            
            <button
                className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer"
                onClick={() => {
                    setShowModal(true);
                }}
            >
                Lihat Jejak
            </button>
            <ModalComponent
                show={showModal}
                onClose={() => {
                    setShowModal(false);
                }}
                title="Jejak Orang Hilang"
            >
                <Trace id={p.id} />
            </ModalComponent>
        </Card>
    )
}

export default Detail;
"use client";
import Card from "./Card";
import Input from "./Input";
import { useState } from "react";
import { apiClient } from "@/hooks/useApi";
import Swal from "sweetalert2";
import { useGlobalContext } from "@/hooks/globalprovider";

const AddLostPerson = (p: {
    setShowAddModal: (show: boolean) => void;
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
    }>({
        fullname: '',
        alias: '',
        born_date: '',
        lost_date: '',
        last_condition: '',
        gender: '',
    })

    const addPerson = async () => {
        try {
            await apiClient({
                method: "post",
                url: `/api/lost_people`,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                },
                data: form
            })

            Swal.fire({
                title: "Add Lost Person Successful",
                icon: "success",
                text: "Lost person has been added successfully.",
                confirmButtonText: "OK"
            });
            p.setShowAddModal(false);
            
        } catch (error) {
            console.error("Add Lost Person Failed:", error);
            Swal.fire({
                title: "Add Lost Person Failed",
                icon: "error",
                confirmButtonText: "OK"
            });
        }
    }

    return (
        <Card>
            <h1 className="text-xl font-bold">Tambah Data</h1>
            <hr className="mb-6" />
            <Input
                id="fullname"
                label="Full Name"
                type="text"
                required
                placeholder="Enter full name"
                onChange={(e) => setForm({ ...form, fullname: e.target.value })}
                value={form.fullname}
            />
            <Input
                id="alias"
                label="Alias"
                type="text"
                required
                placeholder="Enter alias"
                onChange={(e) => setForm({ ...form, alias: e.target.value })}
                value={form.alias}
            />
            <Input
                id="born_date"
                label="Born Date"
                type="date"
                required
                placeholder="Enter born date"
                onChange={(e) => setForm({ ...form, born_date: e.target.value })}
                value={form.born_date}
            />
            <Input
                id="lost_date"
                label="Lost Date"
                type="date"
                required
                placeholder="Enter lost date"
                onChange={(e) => setForm({ ...form, lost_date: e.target.value })}
                value={form.lost_date}
            />
            <Input
                id="last_condition"
                label="Last Condition"
                type="text"
                required
                placeholder="Enter last condition"
                onChange={(e) => setForm({ ...form, last_condition: e.target.value })}
                value={form.last_condition}
            />
            <div className="mb-4">
                <label htmlFor="gender" className="block text-gray-700 font-bold mb-2">
                    Gender
                </label>
                <select
                    id="gender"
                    value={form.gender}
                    onChange={(e) => setForm({ ...form, gender: e.target.value })}
                    className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    required
                >
                    <option value="">Select Gender</option>
                    <option value="L">Laki Laki</option>
                    <option value="P">Perempuan</option>
                </select>
            </div>
            <button
                className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer"
                onClick={() => {
                    addPerson();
                }}
            >
                Upload
            </button>
        </Card>
    )
}

export default AddLostPerson;
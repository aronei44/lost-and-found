"use client";
import Card from "./Card";
import Input from "./Input";
import { useEffect, useState } from "react";
import { apiClient } from "@/hooks/useApi";
import Swal from "sweetalert2";
import { useGlobalContext } from "@/hooks/globalprovider";

const Profile = () => {

    const {
        state: {
            token
        }
    } = useGlobalContext();


    const [form, setForm] = useState<{
        full_name: string,
        email: string,
        phone: string,
        address: string,
    }>({
        full_name: '',
        email: '',
        phone: '',
        address: ''
    })

    const handleUpdateProfile = async () => {
        try {
            const response = await apiClient({
                method: "put",
                url: "/api/profile",
                data: {
                    full_name: form.full_name,
                    email: form.email,
                    phone: form.phone,
                    address: form.address
                },
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                }
            })
            setForm({
                full_name: response.full_name,
                email: response.email,
                phone: response.phone,
                address: response.address
            });
            Swal.fire({
                title: "Update Profile Successful",
                icon: "success",
                confirmButtonText: "OK"
            });
        } catch (error) {
            console.error("Login failed:", error);
            Swal.fire({
                title: "Update Profile Failed",
                text: "Sorry, an error occurred during update profile. Please try again.",
                icon: "error",
                confirmButtonText: "OK"
            });
        }
    }

    const getProfile = async () => {
        try {
            const response = await apiClient({
                method: "get",
                url: "/api/profile",
                headers: {
                    Authorization: `Bearer ${token.accessToken}`
                }
            });
            setForm({
                full_name: response.full_name,
                email: response.email,
                phone: response.phone,
                address: response.address
            });
        } catch (error) {
            console.error("Failed to fetch profile:", error);
        }
    }

    useEffect(() => {
        getProfile();
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    return (
        <div className="container mx-auto p-4 mt-20">
            <Card>
                <h1 className="text-6xl font-bold mt-10">Profile</h1>
                <hr className="mb-6"/>
                <Input
                    id="full_name"
                    label="Full Name"
                    type="text"
                    required
                    placeholder="Enter your full name"
                    onChange={(e) => setForm({ ...form, full_name: e.target.value })}
                    value={form.full_name}
                />
                <Input
                    id="email"
                    label="Email"
                    type="email"
                    required
                    placeholder="Enter your email"
                    onChange={(e) => setForm({ ...form, email: e.target.value })}
                    value={form.email}
                />
                <Input
                    id="phone"
                    label="Phone"
                    type="tel"
                    required
                    placeholder="Enter your phone number"
                    onChange={(e) => setForm({ ...form, phone: e.target.value })}
                    value={form.phone}
                />
                <Input
                    id="address"
                    label="Address"
                    type="text"
                    required
                    placeholder="Enter your address"
                    onChange={(e) => setForm({ ...form, address: e.target.value })}
                    value={form.address}
                />
                <button 
                    onClick={handleUpdateProfile}
                    className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer mb-6">
                        Update Profile
                </button>

            </Card>
        </div>
    )
}

export default Profile;
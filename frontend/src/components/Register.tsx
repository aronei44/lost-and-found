"use client";
import Card from "./Card";
import Input from "./Input";
import { useState } from "react";
import { apiClient } from "@/hooks/useApi";
import { useGlobalContext } from "@/hooks/globalprovider";
import Swal from "sweetalert2";

const Register = () => {

    const { action : {
        setActiveState
    }} = useGlobalContext();

    const [form, setForm] = useState<{
        username: string,
        password: string
    }>({
        username: '',
        password: ''
    })

    const handleRegister = async () => {
        try {
            await apiClient({
                method: "post",
                url: "/api/auth/register",
                data: {
                    username: form.username,
                    password: form.password
                }
            })
            setActiveState("login");
            Swal.fire({
                title: "Register Successful",
                text: "You can now login with your credentials.",
                icon: "success",
                confirmButtonText: "OK"
            });
        } catch (error) {
            console.error("Login failed:", error);
            Swal.fire({
                title: "Login Failed",
                text: "Sorry, an error occurred during registration. Please try again.",
                icon: "error",
                confirmButtonText: "OK"
            });
        } finally {
            setForm({
                username: '',
                password: ''
            });
        }
    }

    return (
        <div className="container mx-auto p-4 mt-20">
            <Card>
                <h1 className="text-6xl font-bold mt-10">Register</h1>
                <hr className="mb-6"/>
                <Input
                    id="username"
                    label="Username"
                    type="text"
                    required
                    placeholder="Enter your username"
                    onChange={(e) => setForm({ ...form, username: e.target.value })}
                    value={form.username}
                />
                <Input
                    id="password"
                    label="Password"
                    type="password"
                    required
                    placeholder="Enter your password"
                    onChange={(e) => setForm({ ...form, password: e.target.value })}
                    value={form.password}
                />
                <button 
                    onClick={handleRegister}
                    className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer mb-6">
                        Register
                </button>

                <p>
                    Sudah memiliki akun?{" "}
                    <button
                        type="button"
                        className="cursor-pointer text-blue-600 bg-transparent border-none p-0 m-0"
                        onClick={() => setActiveState("login")}
                    >
                        Login
                    </button>
                </p>
            </Card>
        </div>
    )
}

export default Register;
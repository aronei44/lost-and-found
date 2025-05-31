"use client";
import Card from "./Card";
import Input from "./Input";
import { useState } from "react";
import { apiClient } from "@/hooks/useApi";
import { useGlobalContext } from "@/hooks/globalprovider";

const Login = () => {

    const { action : {
        setAuthenticated,
        setToken,
        setActiveState
    }} = useGlobalContext();

    const [form, setForm] = useState<{
        username: string,
        password: string
    }>({
        username: '',
        password: ''
    })

    const handleLogin = async () => {
        try {
            const response = await apiClient({
                method: "post",
                url: "/api/auth/login",
                data: {
                    username: form.username,
                    password: form.password
                }
            })
            setAuthenticated(true);
            setToken({
                accessToken: response.access_token,
                refreshToken: response.refresh_token
            });
            setActiveState("main");
        } catch (error) {
            setAuthenticated(false);
            setToken({
                accessToken: '',
                refreshToken: ''
            });
            console.error("Login failed:", error);
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
                <h1 className="text-6xl font-bold mt-10">Login</h1>
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
                    onClick={handleLogin}
                    className="bg-blue-500 text-white px-4 py-2 rounded cursor-pointer mb-6">
                        Login
                </button>

                <p>
                    Belum memiliki akun?{" "}
                    <button
                        type="button"
                        className="cursor-pointer text-blue-600 bg-transparent border-none p-0 m-0"
                        onClick={() => setActiveState("register")}
                    >
                        Register
                    </button>
                </p>
            </Card>
        </div>
    )
}

export default Login;
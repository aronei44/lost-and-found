import axios from 'axios';
import { serverKey } from './useServerKey';


export const apiClient = async (p: {
    method: "get" | "post" | "put" | "delete";
    url: string;
    data?: Record<string, unknown>;
    params?: Record<string, unknown>;
    headers?: Record<string, string>;
    baseUrl?: string;
}) => {
    let baseURL = p.baseUrl;
    baseURL ??= await serverKey("API_BASE_URL");
    try {
        const request = await axios({
            baseURL,
            method: p.method,
            url: p.url,
            data: p.data,
            params: p.params,
            headers: {
                "Content-Type": "application/json",
                ...p.headers,
            },
        })
        return request.data;
    } catch (error) {
        if (axios.isAxiosError(error)) {
            // Handle Axios error
            console.error("Axios error:", error.message);
            throw new Error(`API request failed: ${error.message}`);
        } else {
            // Handle non-Axios error
            console.error("Unexpected error:", error);
            throw new Error("An unexpected error occurred");
        }
    }
}
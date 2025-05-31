"use server";

export const serverKey = async (key: string) => {
    return process.env[key] ?? "";
}

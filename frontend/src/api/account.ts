import { useUserStore } from "@/stores/user";
import type { EditUserData, User } from "@/types/user";
import client from "@/api/client";

export const registerAccount = (args: {
    username: string;
    password: string;
    identity?: string;
}) => {
    return client.post("/api/register", args);
};

export const loginAccount = (data: { username: string; password: string }) => {
    return client.post("/api/login", data);
};

export const fetchUsers = (limit: number) => {
    // 使用已配置的 `client`（包含 baseURL 与自动注入的 Authorization header）
    return client.get("/api/users", { params: { limit } });
};

export const deleteUser = (userId: string) => {
    // client 会自动注入 Authorization header
    return client.delete(`/api/users/${userId}`);
};

export const EditAccount = (payload: EditUserData) => {
    return client.put("/api/editAccount", payload);
};

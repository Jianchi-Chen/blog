import { useUserStore } from "@/stores/user";
import type { User } from "@/types/user";
import axios from "axios";

export const registerAccount = (args: {
    username: string;
    password: string;
}) => {
    return axios.post("/api/register", args);
};

export const loginAccount = (data: { username: string; password: string }) => {
    return axios.post("/api/login", data);
};

export const fetchUsers = (limit: number) => {
    // 使用已配置的 `client`（包含 baseURL 与自动注入的 Authorization header）
    return axios.get("/api/users", { params: { limit } });
};

export const deleteUser = (userId: string) => {
    const userStore = useUserStore();
    return axios.delete(`/api/users/${userId}`, {
        headers: {
            Authorization: `Bearer ${userStore.token}`,
        },
    });
};

import { useUserStore } from "@/stores/user";
import type { EditUserData, User } from "@/types/user";
import client from "@/api/client";
import { isTauri, invoke } from "@tauri-apps/api/core";

export const registerAccount = async (args: {
    username: string;
    password: string;
    identity?: string;
}) => {
    if (isTauri()) {
        // 在 Tauri 中直接调用后端命令
        const data = await invoke("register", { userInfo: args });
        return {
            data,
            status: 200,
            statusText: "OK",
            headers: {},
            config: {},
        } as any;
    }
    return client.post("/api/register", args);
};

export const loginAccount = async (data: {
    username: string;
    password: string;
}) => {
    if (isTauri()) {
        // 在 Tauri 中直接调用后端命令
        const result = await invoke("login", { credentials: data });
        return {
            data: result,
            status: 200,
            statusText: "OK",
            headers: {},
            config: {},
        } as any;
    }
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

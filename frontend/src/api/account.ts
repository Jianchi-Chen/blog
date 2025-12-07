import { useUserStore } from "@/stores/user";
import { useAppStore } from "@/stores/app";
import type { EditUserData } from "@/types/user";
import client from "@/api/client";
import { invoke } from "@tauri-apps/api/core";

export const registerAccount = async (args: {
    username: string;
    password: string;
    identity?: string;
}) => {
    const app = useAppStore();
    
    if (app.isTauri) {
        const data = await invoke("register", { userInfo: args });
        return { data };
    }
    
    return client.post("/api/register", args);
};

export const loginAccount = async (data: {
    username: string;
    password: string;
}) => {
    const app = useAppStore();
    
    if (app.isTauri) {
        const result = await invoke("login", { credentials: data });
        return { data: result };
    }
    
    return client.post("/api/login", data);
};

export const fetchUsers = async (limit: number) => {
    const app = useAppStore();
    const user = useUserStore();
    
    if (app.isTauri) {
        const data = await invoke("get_users", {
            token: user.token,
            limit
        });
        return { data };
    }
    
    // 使用已配置的 `client`（包含 baseURL 与自动注入的 Authorization header）
    return client.get("/api/users", { params: { limit } });
};

export const deleteUser = async (userId: string) => {
    const app = useAppStore();
    const user = useUserStore();
    
    if (app.isTauri) {
        const data = await invoke("delete_user", {
            token: user.token,
            userId
        });
        return { data };
    }
    
    // client 会自动注入 Authorization header
    return client.delete(`/api/users/${userId}`);
};

export const EditAccount = async (payload: EditUserData) => {
    const app = useAppStore();
    const user = useUserStore();
    
    if (app.isTauri) {
        const data = await invoke("edit_account", {
            token: user.token,
            payload
        });
        return { data };
    }
    
    return client.put("/api/editAccount", payload);
};

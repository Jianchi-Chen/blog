import { useUserStore } from "@/stores/user";
import axios from "axios";
import { API_BASE_URL } from "@/config";

// 所有 API 请求都会自动继承这些统一配置, 便于维护
const client = axios.create({
    baseURL: API_BASE_URL, // 全局base后端地址
    timeout: 5000, // 请求5s超时
});

// 为这个 axios client 添加了一个 请求拦截器（request interceptor）
// 作用是在发请求前，自动读取当前用户的 token，并加到请求头中！
client.interceptors.request.use((config) => {
    // 从pinia中读取用户的token, useUserStore本身是一个函数
    const user = useUserStore();
    if (user.token) {
        // "Bearer" 是 JWT token 的一种标准格式写法。
        config.headers.Authorization = `Bearer ${user.token}`;
    }

    return config;
});

export default client;

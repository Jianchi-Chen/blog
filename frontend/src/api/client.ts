import { useUserStore } from "@/stores/user";
import axios from "axios";

// 所有 API 请求都会自动继承这些统一配置, 便于维护
const client = axios.create({
  baseURL: "http://localhost:5173/", // 全局base后端地址
  timeout: 5000, // 请求5s超时
});

// 为这个 client 添加了一个 请求拦截器（request interceptor）
// 作用是在发请求前，自动读取当前用户的 token，并加到请求头中！
// 这里的 config 是每次请求前 axios 自动传入的请求配置对象(这次 axios 请求的所有配置（如 url、method、headers、data 等）)
// 你可以修改它，比如加 headers、修改 baseURL、加参数等等
client.interceptors.request.use((config) => {
  // 从pinia中读取用户的token, useUserStore本身是一个函数
  const user = useUserStore();
  if (user.token) {
    // “Bearer” 是 JWT token 的一种标准格式写法。
    config.headers.Authorization = "Bearer ${user.token}";
  }

  return config;
});

export default client;

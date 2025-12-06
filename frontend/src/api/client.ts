import { useUserStore } from "@/stores/user";
import axios, { type AxiosRequestConfig, type AxiosResponse } from "axios";
import { API_BASE_URL } from "@/config";
import { isTauri, invoke } from "@tauri-apps/api/core";

// 所有 API 请求都会自动继承这些统一配置, 便于维护
const axiosClient = axios.create({
    baseURL: API_BASE_URL, // 全局base后端地址
    timeout: 5000, // 请求5s超时
});

// 为这个 axios client 添加了一个 请求拦截器（request interceptor）
// 作用是在发请求前，自动读取当前用户的 token，并加到请求头中！
axiosClient.interceptors.request.use((config) => {
    // 从pinia中读取用户的token, useUserStore本身是一个函数
    const user = useUserStore();
    if (user.token) {
        // "Bearer" 是 JWT token 的一种标准格式写法。
        config.headers.Authorization = `Bearer ${user.token}`;
    }

    return config;
});

// Tauri invoke 包装函数
async function tauriRequest<T = any>(
    method: string,
    url: string,
    config?: AxiosRequestConfig
): Promise<AxiosResponse<T>> {
    const user = useUserStore();

    // 构建请求参数
    const requestData: any = {
        method: method.toUpperCase(),
        url,
        __token: user.token || "",
    };

    // 添加查询参数
    if (config?.params) {
        requestData.params = config.params;
    }

    // 添加请求体数据
    if (config?.data) {
        requestData.data = config.data;
    }

    try {
        const data = await invoke<T>("http_request", requestData);

        // 返回类似 axios 的响应格式
        return {
            data,
            status: 200,
            statusText: "OK",
            headers: {},
            config: config as any,
        } as AxiosResponse<T>;
    } catch (error) {
        throw {
            response: {
                data: error,
                status: 500,
                statusText: "Internal Server Error",
            },
        };
    }
}

// 统一的 client 接口，根据环境自动选择
const client = {
    get<T = any>(
        url: string,
        config?: AxiosRequestConfig
    ): Promise<AxiosResponse<T>> {
        if (isTauri()) {
            return tauriRequest<T>("GET", url, config);
        }
        return axiosClient.get<T>(url, config);
    },

    post<T = any>(
        url: string,
        data?: any,
        config?: AxiosRequestConfig
    ): Promise<AxiosResponse<T>> {
        if (isTauri()) {
            return tauriRequest<T>("POST", url, { ...config, data });
        }
        return axiosClient.post<T>(url, data, config);
    },

    put<T = any>(
        url: string,
        data?: any,
        config?: AxiosRequestConfig
    ): Promise<AxiosResponse<T>> {
        if (isTauri()) {
            return tauriRequest<T>("PUT", url, { ...config, data });
        }
        return axiosClient.put<T>(url, data, config);
    },

    delete<T = any>(
        url: string,
        config?: AxiosRequestConfig
    ): Promise<AxiosResponse<T>> {
        if (isTauri()) {
            return tauriRequest<T>("DELETE", url, config);
        }
        return axiosClient.delete<T>(url, config);
    },

    patch<T = any>(
        url: string,
        data?: any,
        config?: AxiosRequestConfig
    ): Promise<AxiosResponse<T>> {
        if (isTauri()) {
            return tauriRequest<T>("PATCH", url, { ...config, data });
        }
        return axiosClient.patch<T>(url, data, config);
    },
};

export default client;

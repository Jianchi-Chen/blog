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

// URL 路径到 Tauri 命令的映射
interface TauriCommandMapping {
    command: string;
    paramsMapper?: (url: string, config?: AxiosRequestConfig) => any;
}

function getTauriCommand(method: string, url: string, config?: AxiosRequestConfig): TauriCommandMapping | null {
    const user = useUserStore();
    
    // GET 请求映射
    if (method === "GET") {
        if (url === "/articles") {
            return {
                command: "get_articles",
                paramsMapper: (_, cfg) => ({
                    identity: cfg?.params?.identity || "visitor",
                    condition: cfg?.params?.condition,
                }),
            };
        }
        if (url.match(/^\/article\/(.+)$/)) {
            const id = url.match(/^\/article\/(.+)$/)?.[1];
            return {
                command: "get_article_by_id",
                paramsMapper: () => ({ id }),
            };
        }
        if (url.match(/^\/comments\/(.+)$/)) {
            const articleId = url.match(/^\/comments\/(.+)$/)?.[1];
            return {
                command: "get_comments",
                paramsMapper: () => ({
                    articleId,
                    token: user.token || null,
                }),
            };
        }
        if (url.match(/^\/suggestions\/(.+)$/)) {
            const keyword = url.match(/^\/suggestions\/(.+)$/)?.[1];
            return {
                command: "get_suggestions",
                paramsMapper: () => ({ keyword }),
            };
        }
        if (url === "/api/users") {
            return {
                command: "get_users",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    limit: cfg?.params?.limit || 100,
                }),
            };
        }
    }
    
    // POST 请求映射
    if (method === "POST") {
        if (url === "/api/article") {
            return {
                command: "create_article",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    articleData: cfg?.data,
                }),
            };
        }
        if (url === "/api/comment") {
            return {
                command: "post_comment",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    commentData: {
                        article_id: cfg?.data?.article_id,
                        content: cfg?.data?.content,
                        parent_id: cfg?.data?.parent_id,
                    },
                }),
            };
        }
    }
    
    // PUT 请求映射
    if (method === "PUT") {
        if (url === "/api/comment/like") {
            return {
                command: "like_comment",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    payload: {
                        comment_id: cfg?.data?.comment_id,
                    },
                }),
            };
        }
        if (url.match(/^\/api\/article\/(.+)$/)) {
            const id = url.match(/^\/api\/article\/(.+)$/)?.[1];
            return {
                command: "update_article",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    id,
                    articleData: cfg?.data,
                }),
            };
        }
        if (url === "/api/editAccount") {
            return {
                command: "edit_account",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    payload: cfg?.data,
                }),
            };
        }
    }
    
    // DELETE 请求映射
    if (method === "DELETE") {
        if (url.match(/^\/api\/article\/(.+)$/)) {
            const id = url.match(/^\/api\/article\/(.+)$/)?.[1];
            return {
                command: "delete_article",
                paramsMapper: () => ({
                    token: user.token,
                    id,
                }),
            };
        }
        if (url.match(/^\/api\/users\/(.+)$/)) {
            const userId = url.match(/^\/api\/users\/(.+)$/)?.[1];
            return {
                command: "delete_user",
                paramsMapper: () => ({
                    token: user.token,
                    userId,
                }),
            };
        }
        if (url.match(/^\/comment\/(.+)$/)) {
            const commentId = url.match(/^\/comment\/(.+)$/)?.[1];
            return {
                command: "delete_comment",
                paramsMapper: () => ({
                    token: user.token,
                    commentId,
                }),
            };
        }
    }
    
    // PATCH 请求映射
    if (method === "PATCH") {
        if (url.match(/^\/api\/article\/(.+)$/)) {
            const id = url.match(/^\/api\/article\/(.+)$/)?.[1];
            return {
                command: "toggle_article_status",
                paramsMapper: (_, cfg) => ({
                    token: user.token,
                    id,
                    toggle: cfg?.data?.toggle,
                }),
            };
        }
    }
    
    return null;
}

// Tauri invoke 包装函数
async function tauriRequest<T = any>(
    method: string,
    url: string,
    config?: AxiosRequestConfig
): Promise<AxiosResponse<T>> {
    // 尝试映射到 Tauri 命令
    const mapping = getTauriCommand(method.toUpperCase(), url, config);
    
    if (mapping) {
        try {
            const params = mapping.paramsMapper ? mapping.paramsMapper(url, config) : {};
            const data = await invoke<T>(mapping.command, params);
            
            // 返回类似 axios 的响应格式
            return {
                data,
                status: 200,
                statusText: "OK",
                headers: {},
                config: config as any,
            } as AxiosResponse<T>;
        } catch (error) {
            console.error(`Tauri command ${mapping.command} failed:`, error);
            throw {
                response: {
                    data: error,
                    status: 500,
                    statusText: "Internal Server Error",
                },
            };
        }
    }
    
    // 如果没有映射，回退到 http_request（用于未实现的端点）
    const user = useUserStore();
    const requestData: any = {
        method: method.toUpperCase(),
        url,
        __token: user.token || "",
    };

    if (config?.params) {
        requestData.params = config.params;
    }

    if (config?.data) {
        requestData.data = config.data;
    }

    try {
        const data = await invoke<T>("http_request", requestData);
        return {
            data,
            status: 200,
            statusText: "OK",
            headers: {},
            config: config as any,
        } as AxiosResponse<T>;
    } catch (error) {
        console.error(`HTTP request failed:`, error);
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

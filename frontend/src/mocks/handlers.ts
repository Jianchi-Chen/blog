// src/mocks/handlers.ts 定义 mock 接口
import { http, HttpResponse } from "msw";

export const handlers = [
  http.post("http://localhost:5173/login", () => {
    return HttpResponse.json({
      token: "mock-jwt-token",
      username: "admin",
    });
  }),
];

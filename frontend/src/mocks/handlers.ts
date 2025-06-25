// src/mocks/handlers.ts 定义 mock 接口
import { http, HttpResponse } from "msw";
import { useRouter } from "vue-router";

export const handlers = [
  // /login 登录
  http.post("http://localhost:5173/login", () => {
    return HttpResponse.json({
      token: "mock-jwt-token",
      username: "admin",
    });
  }),

  // 获取文章列表
  http.get("http://localhost:5173/articles", () => {
    return HttpResponse.json([
      {
        id: 1,
        title: "Vue 3 从入门到实战",
        summary: "学习 Vue 3 的最佳实践",
        created_at: "2024-06-01",
      },
      {
        id: 2,
        title: "深入理解 Pinia",
        summary: "下一代状态管理工具",
        created_at: "2024-06-10",
      },
      {
        id: 3,
        title: "Tailwind CSS 快速指南",
        summary: "从零上手到精通",
        created_at: "2024-06-15",
      },
    ]);
  }),

  // 获取文章详情, 使用动态路由
  http.get("/article/:id", ({ params }) => {
    const { id } = params; // 解构 id

    return HttpResponse.json({
      id,
      title: `文章标题 ${id}`,
      content: `这里是文章 ${id} 的详细内容，使用 Markdown 格式编写`,
      created_at: "2024-06-01",
    });
  }),
];

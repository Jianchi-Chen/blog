// src/mocks/handlers.ts 定义 mock 接口
import { fetchArticles } from "@/api/article";
import { http, HttpResponse } from "msw";
import { error } from "naive-ui/es/_utils/naive/warn";
import type { StringMappingType } from "typescript";
import { ref } from "vue";
import { useRouter } from "vue-router";

export const handlers = [
  // /login 登录
  http.post("http://localhost:5173/login", async ({ request }) => {
    // 解包请求内容, 动态响应传入的用户名字符串
    const body = (await request.json()) as {
      username: string;
      password: string;
    };
    const { username } = body;

    return HttpResponse.json({
      token: "mock-jwt-token",
      username,
    });
  }),

  // 获取文章列表
  http.get("/articles", () => {
    return HttpResponse.json(articles);
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

  // 创建文章
  http.post("/article", async ({ request }) => {
    const data = (await request.json()) as Record<string, any>;

    if (
      typeof data !== "object" ||
      !data.title ||
      !data.content ||
      typeof data.title !== "string" ||
      typeof data.content !== "string"
    ) {
      return HttpResponse.json(
        { error: "title and content can't empty" },
        { status: 400 }
      );
    }

    const newArticle = {
      id: articles.length + 1,
      ...data,
    };
    articles.push(newArticle);
    return HttpResponse.json(newArticle);
  }),

  // 删除文章
  http.post("/articleDelete", async ({ request }) => {
    const id = await request.json();
    articles = articles.filter((article) => article.id !== id);
    return HttpResponse.json({ success: true });
  }),
];

// 模拟数据库，只存于内存中
let articles: any[] = [
  {
    id: 1,
    title: "Vue 3 从入门到实战",
    summary: "学习 Vue 3 的最佳实践",
    status: "published",
    created_at: "2024-06-01",
  },
  {
    id: 2,
    title: "深入理解 Pinia",
    summary: "下一代状态管理工具",
    status: "published",
    created_at: "2024-06-10",
  },
  {
    id: 3,
    title: "Tailwind CSS 快速指南",
    summary: "从零上手到精通",
    status: "draft",
    created_at: "2024-06-15",
  },
];

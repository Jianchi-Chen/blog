// src/mocks/handlers.ts 定义 mock 接口
import { fetchArticles } from "@/api/article";
import { ArticleSchema, type Article } from "@/types/article";
import { http, HttpResponse } from "msw";
import { error } from "naive-ui/es/_utils/naive/warn";
import type { StringMappingType } from "typescript";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { uuidv7 } from "uuidv7";
import z, { number } from "zod";
import { format } from "date-fns"; // 格式化展示

export const handlers = [
  // /login 登录
  http.post("/login", async ({ request }) => {
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
  http.get("/articles", ({ request }) => {
    // GET请求的参数在url中，所以要从url中拿
    const url = new URL(request.url); // 把字符串转换成 URL 对象
    const identity = url.searchParams.get("identity"); // ✅ 可以正常访问查询参数

    if (identity === "admin") {
      return HttpResponse.json(articles.value);
    } else if (identity === "visitor") {
      return HttpResponse.json(
        articles.value.filter((art) => art.status === "published")
      );
    } else {
      return HttpResponse.json(
        articles.value.filter((art) => art.status === "published")
      );
    }
  }),

  // 获取文章详情, 使用动态路由
  http.get("/article/:id", ({ params }) => {
    const { id } = params; // 解构 id
    const find_article = articles.value.find((a) => a.id == id);
    if (!find_article) {
      return HttpResponse.json({
        error: "文章不存在",
      });
    }

    return HttpResponse.json(find_article);
  }),

  // 新建文章
  http.post("/article", async ({ request }) => {
    const json = await request.json(); // 未经验证的生肉
    const result = ArticleSchema.safeParse(json); // 验证，需要数据结构的值而非类型
    if (!result.success) {
      return HttpResponse.json(
        { error: "title and content can't empty" },
        { status: 400 }
      );
    }

    // 格式化时间
    const newArticle = {
      id: uuidv7(), // uuid生成唯一且有序的id
      ...result.data,
      created_at: result.data.created_at ?? new Date().toISOString(), // nullish表达式，如果是空值就给一个时间
    };
    const formattedTime = format(
      new Date(newArticle.created_at),
      "yyyy-MM-dd HH:mm"
    );
    newArticle.created_at = formattedTime;

    // 简介
    if (!newArticle.summary) {
      newArticle.summary = result.data.title;
    }

    articles.value.push(newArticle);
    return HttpResponse.json(newArticle);
  }),

  // 删除文章
  http.delete("/article/:id", async ({ params }) => {
    const id = Number(params.id);
    articles.value = articles.value.filter((article) => article.id !== id);
    return HttpResponse.json({ message: "删除成功" });
  }),

  // 修改文章
  http.put("/article/:id", async ({ request, params }) => {
    const id = Number(params.id);
    const res = (await request.json()) as Article;
    const index = articles.value.findIndex((a) => a.id === id);

    if (index !== -1 && typeof res == "object" && res !== null) {
      articles.value[index] = { ...articles.value[index], ...res };
    }

    return HttpResponse.json(articles.value[index]);
  }),

  // 更变文章状态
  http.patch("/article/:id", async ({ request, params }) => {
    const id = Number(params.id);
    // 类型断言
    const data = (await request.json()) as { toggle?: string };
    const toggle = data.toggle;

    const index = articles.value.findIndex((a) => a.id == id);

    if (index !== -1 && articles.value[index].status !== toggle) {
      articles.value[index].status = toggle;
    }

    return HttpResponse.json(articles.value[index]);
  }),
];

// 模拟数据库，只存于内存中
let articles = ref<any[]>([
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
]);

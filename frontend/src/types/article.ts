import { z } from "zod";

// Zod 是一个 TypeScript 友好的 schema 校验库，可以帮你声明文章结构并自动验证。
export const ArticleSchema = z.object({
  id: z.number().optional(),
  title: z.string().min(1, "不能为空"), // 字符串最小长度为1
  content: z.string().min(1, "不能为空"),
  summary: z.string().optional(),
  created_at: z.iso.datetime().optional(), // 推荐用 ISO 时间字符串
  update_at: z.string().optional(),
  update_count: z.number().optional(),
  author_name: z.string().optional(),
  status: z.enum(["published", "draft", "archived"]).optional(), // 枚举
  views: z.number().min(0).optional(), // 非负数
});

export type Article = z.infer<typeof ArticleSchema>; // 自动推导类型

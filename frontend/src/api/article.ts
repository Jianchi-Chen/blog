import type { Article } from "@/types/article";
import axios from "./client";

// 获取文章列表
export const fetchArticles = (identity: string) => {
  // GET请求中，第二个参数需要写在params里，params 是专门用来指定 URL 查询参数的字段，它的值必须是一个对象
  return axios.get("/articles", { params: { identity } });
};

// 获取文章详情
// 从表单、输入框获得的值通常为string, 所以id的类型设置为两种
// 模板字面量使用反斜杠``包裹，否则不识别！
export const fetchArticleById = (id: number | string) => {
  return axios.get(`/article/${id}`);
};

// 新建文章
export const publishArticle = (arg: Article) => {
  return axios.post("/article/", arg);
};

// 文章数据结构
interface article_model {
  title?: string;
  content?: string;
  created_at?: string;
}

// 修改文章
export const updateArticle = (id: number, data: Article) => {
  return axios.put(`/article/${id}`, data);
};

// 删除文章
export const deleteArticle = (id: number) => {
  return axios.delete(`/article/${id}`);
};

// 转换文章状态
export const toggleStatus = (id: number, toggle: string) => {
  return axios.patch(`/article/${id}`, { toggle });
};

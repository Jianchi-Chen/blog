import axios from "./client";

// 获取文章列表
export const fetchArticles = () => {
  return axios.get("/articles");
};

// 获取文章详情
// 从表单、输入框获得的值通常为string, 所以id的类型设置为两种
// 模板字面量使用反斜杠``包裹，否则不识别！
export const fetchArticleById = (id: number | string) => {
  return axios.get(`/article/${id}`);
};

import type { Article } from "@/types/article";
import axios from "./client";
import { useMessage } from "naive-ui";
import { setup } from "naive-ui/es/radio/src/use-radio";

// 封装api函数返回promis，故前端调用时使用async/await即可

// 获取文章列表
export const fetchArticles = (identity: string, condition?: string) => {
  // GET请求中，第二个参数需要写在params里，params 是专门用来指定 URL 查询参数的字段，它的值必须是一个对象
  return axios.get("/articles", { params: { identity, condition } });
};

// 获取文章详情
// 从表单、输入框获得的值通常为string, 所以id的类型设置为两种
// 模板字面量使用反斜杠``包裹，否则不识别！
export const fetchArticleById = (id: Article["id"]) => {
  return axios.get(`/article/${id}`);
};

// 新建文章
export const publishArticle = (arg: Article) => {
  return axios.post("/api/article", arg);
};

// 修改文章
export const updateArticle = (id: Article["id"], data: Article) => {
  return axios.put(`/api/article/${id}`, data);
};

// 删除文章
export const deleteArticle = (id: Article["id"]) => {
  return axios.delete(`/api/article/${id}`);

  //? message()无法脱离setup()使用，下列代码改为前端使用
  //   const message = useMessage();
  // if (res && res.data.message == "删除成功") {
  //   message.success("Delete succeeded");
  // } else if (res && res.data.message == "删除失败") {
  //   message.error("Delete failed");
  // } else {
  //   message.warning("operation failed");
  // }
};

// 转换文章状态
export const toggleStatus = (id: Article["id"], toggle: string) => {
  return axios.patch(`/api/article/${id}`, { toggle });
};

// 获取建议
export const fetchSuggestions = (keyword: string) => {
  return axios.get(`/suggestions/${keyword}`);
};

export const fetchArticleByConditions = (condition: string) => {
  return axios.get(`/article/${condition}`);
};

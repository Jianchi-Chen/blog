import axios from "axios";

// 发表评论
export const postComment = (articleId: string, content: string, user: string) => {
  return axios.post("/comment", { articleId, content, user });
};

// 获取评论
export const fetchComments = (articleId: string) => {
  return axios.get(`/comments/${articleId}`);
};

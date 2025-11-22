import axios from "./client";

// 发表评论
export const postComment = (
    article_id: string,
    content: string,
    user_id: string
) => {
    return axios.post("/api/comment", { article_id, user_id, content });
};

// 删除评论
export const DeleteComment = (CommentId: string) => {
    console.log("CommentId:", CommentId);
    return axios.delete(`/comment/${CommentId}`);
};

// 获取评论
export const fetchComments = (articleId: string) => {
    return axios.get(`/comments/${articleId}`);
};

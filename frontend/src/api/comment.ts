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
    return axios.delete(`/comment/${CommentId}`);
};

// 获取评论
export const fetchComments = (articleId: string) => {
    return axios.get(`/comments/${articleId}`);
};

// 获取评论点赞情况
export const fetchCommentsLike = (articleId: string) => {
    return axios.get("/api/comments/like", {
        params: { article_id: articleId },
    });
};

// 更新评论点赞数
export const updateCommentLike = (commentId: string, userToken: string) => {
    return axios.put("/api/comment/like", {
        comment_id: commentId,
    });
};

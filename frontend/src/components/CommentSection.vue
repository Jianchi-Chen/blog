<template>
    <n-layout class="w-[95%] comment-container">
        <div ref="commentTitle">
            <n-h2>评论区</n-h2>
        </div>
        <!-- 评论框 -->
        <div
            :class="{ fixedBottom: isFixed }"
            :style="{
                left: isFixed
                    ? articleStore.expandFolder
                        ? '64px'
                        : '240px'
                    : 'auto',

                background: articleStore.osTheme === false ? '#fff' : '#1f1f1f',
            }"
        >
            <EntryCommentBar
                ref="entryRef"
                :articleId="articleId"
                @success="postedComment"
            />
        </div>

        <n-alert v-if="!ifComment" type="info" show-icon class="mb-4">
            暂无评论，快来抢沙发吧~
        </n-alert>

        <n-card v-for="comment in comments" :key="comment.comment_id">
            <!-- 主评论 -->
            <div v-if="comment.parent_id == null">
                <n-p>{{ comment.content }}</n-p>
                <n-flex inline :wrap="false" justify="start" align="center">
                    <!-- 点赞评论 -->
                    <n-button
                        @click="
                            likeComment(
                                comment.user,
                                comment.comment_id,
                                'main'
                            )
                        "
                        :bordered="false"
                    >
                        <div v-if="comment.liked_by_me === 0">
                            <n-icon size="20"
                                ><FavoriteBorderOutlined
                            /></n-icon>
                        </div>

                        <div v-else>
                            <n-icon size="20"><FavoriteOutlined /></n-icon>
                        </div>

                        {{ comment.like_count }}
                    </n-button>

                    <n-text type="success"
                        >来自: {{ comment.user }} |
                        {{ comment.created_at }}</n-text
                    >
                    <n-button
                        @click="
                            respondComment(comment.user, comment.comment_id)
                        "
                    >
                        回复</n-button
                    >
                    <n-button
                        v-if="userStore.identity == 'admin'"
                        @click="handlerDeleteComment(comment.comment_id)"
                    >
                        删除</n-button
                    >
                </n-flex>
            </div>

            <!-- 子评论 -->
            <div
                v-for="child in comment.children"
                :key="child.comment_id"
                class="child-comment"
            >
                <n-p>{{ child.content }}</n-p>

                <n-flex inline :wrap="false" justify="start" align="center">
                    <n-button
                        @click="
                            likeComment(child.user, child.comment_id, 'child')
                        "
                        :bordered="false"
                    >
                        <div v-if="child.liked_by_me === 0">
                            <n-icon size="20"
                                ><FavoriteBorderOutlined
                            /></n-icon>
                        </div>
                        <div v-else>
                            <n-icon size="20"><FavoriteOutlined /></n-icon>
                        </div>
                        {{ child.like_count }}
                    </n-button>

                    <n-text type="success">
                        来自: {{ child.user }} | {{ child.created_at }}
                    </n-text>

                    <n-button
                        @click="respondComment(child.user, comment.comment_id)"
                    >
                        回复
                    </n-button>

                    <n-button
                        v-if="userStore.identity == 'admin'"
                        @click="handlerDeleteComment(child.comment_id)"
                    >
                        删除
                    </n-button>
                </n-flex>
            </div>
        </n-card>
    </n-layout>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import {
    NLayout,
    useDialog,
    NFlex,
    NButton,
    NCard,
    useMessage,
    NAlert,
    NH2,
    NText,
    NP,
    c,
} from "naive-ui";
import { computed, onMounted, onUnmounted, ref, type Ref } from "vue";
import { DeleteComment, fetchComments, updateCommentLike } from "@/api/comment";
import { useRoute } from "vue-router";
import { useArticleStore } from "@/stores/article";
import EntryCommentBar from "./EntryCommentBar.vue";
import { FavoriteBorderOutlined } from "@vicons/material";
import { FavoriteOutlined } from "@vicons/material";
import { useAppStore } from "@/stores/app";

const commentTitle = ref<HTMLElement | null>(null);
const isFixed = ref(false);
const message = useMessage();
const userStore = useUserStore();
const articleStore = useArticleStore();
const route = useRoute();
const articleId = route.params.id as string;
const comments = ref<any[]>([]);
const ifComment = ref(false);
const dialog = useDialog();
const entryRef = ref<any>(null);
const likedComments = ref<string[]>([]);
const isLiked = ref([]);
const AppStore = useAppStore();

// 初始化
const loadComments = async () => {
    try {
        // 获取评论列表
        const res = await fetchComments(articleId);
        console.log("Fetched comments:", res);
        
        // 统一提取数据数组
        let commentsData: any[];
        if (AppStore.isTauri) {
            // Tauri: { data: Array }
            commentsData = res.data as any[];
        } else {
            // Web: AxiosResponse { data: { comments: Array } }
            commentsData = (res as any).data.comments;
        }
        
        comments.value = buildCommentsTree(commentsData);
        ifComment.value = comments.value.length > 0;
    } catch (error) {
        console.error("Error fetching comments:", error);
        message.error("加载评论失败，请稍后重试");
    }
};

// 构建评论树
const buildCommentsTree = (commentsData: any) => {
    const map = new Map();
    const roots: any[] = [];

    // 确保传入的是数组
    const comments = Array.isArray(commentsData) ? commentsData : [];
    console.log("comments:", comments);

    // 为每个评论初始化 children 数组
    comments.forEach((c) => {
        map.set(c.comment_id, { ...c, children: [] });
    });

    // 构建树形结构
    comments.forEach((c) => {
        if (c.parent_id == null) {
            // 一级评论（主评论）直接加入 roots
            roots.push(map.get(c.comment_id));
        } else {
            // 所有非一级评论都作为二级评论加到对应父评论的 children 中
            const parent = map.get(c.parent_id);
            if (parent) {
                parent.children.push(map.get(c.comment_id));
            }
        }
    });

    return roots;
};

// 发布后刷新评论列表
const postedComment = async () => {
    await loadComments();
};

// 删除评论
const handlerDeleteComment = async (commentid: string) => {
    dialog.warning({
        title: "确认删除?",
        content: "此操作将永久删除该评论，是否继续?",
        positiveText: "确定",
        negativeText: "取消",
        onPositiveClick: async () => {
            await DeleteComment(commentid);
            loadComments();
        },
    });
};

// 回复评论
const respondComment = (username: string, parent_id: string) => {
    entryRef.value?.setComment?.(`@${username} `, parent_id);
};

onMounted(() => {
    // 创建一个观察器，用来监听某个 DOM 元素是否进入或离开视口
    const observer = new IntersectionObserver(
        ([entry]) => {
            // entry.isIntersecting 表示元素是否在视口内
            // 如果标题离开视口顶部，就把 isFixed 设置为 true
            isFixed.value = !entry.isIntersecting;
        },
        { threshold: 0 } // 阈值：0 表示只要元素有任何部分进入/离开视口就触发
    );

    // 开始观察标题这个 DOM 元素
    if (commentTitle.value) {
        observer.observe(commentTitle.value);
    }

    // 组件卸载时断开观察器，避免内存泄漏
    onUnmounted(() => observer.disconnect());

    loadComments();
});

// 点赞评论
const likeComment = async (
    username: string,
    commentId: string,
    mode: string
) => {
    const utoken = userStore.token;
    if (!utoken) {
        message.warning("请先登录后再点赞");
        return;
    }

    const res = await updateCommentLike(commentId, utoken);
    if (
        AppStore.isTauri
            ? (res.data as any)?.message !== "liked" &&
              (res.data as any)?.message !== "unliked"
            : (res as any).status !== 200
    ) {
        message.error("点赞失败，请稍后重试");
        return;
    }

    // 局部更新
    let target: any = null;
    if (mode === "main") {
        target = comments.value.find((c) => c.comment_id === commentId);
    } else {
        target = comments.value
            .flatMap((c) => c.children)
            .find((child: any) => child.comment_id === commentId);
    }

    if (!target) {
        message.error("找不到对应的评论");
        return;
    }

    if (res.data.like_or_unlike === "liked") {
        target.like_count += 1;
        target.liked_by_me = 1;
        message.success("点赞成功");
    } else {
        target.like_count -= 1;
        target.liked_by_me = 0;
        message.info("取消点赞");
    }
};
</script>

<style scoped>
.fixedBottom {
    position: fixed;
    bottom: 0;
    right: 20px;
    z-index: 1000;
    transition: left 0.3s ease;
}

/* 防止评论框遮住最后一条评论，添加底部间距 */
.comment-container {
    padding-bottom: 280px;
}

/* 子评论样式 - 贴合 Naive UI 风格 */
.child-comment {
    margin-top: 12px;
    margin-bottom: 12px;
    margin-left: 32px;
    padding: 12px 16px;
    border-left: 3px solid var(--n-border-color, #e0e0e0);
    border-radius: 2px;
    background: var(--n-color, #fafafa);
    transition: background-color 0.3s ease, border-color 0.3s ease;
}
</style>

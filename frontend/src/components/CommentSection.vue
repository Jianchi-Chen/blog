<template>
    <n-layout class="w-[95%]">
        <div ref="commentTitle">
            <n-h2>评论区</n-h2>
        </div>
        <div :class="{ fixedBottom: isFixed }">
            <EntryCommentBar :articleId="articleId" @success="postedComment" />
        </div>

        <n-alert v-if="!ifComment" type="info" show-icon class="mb-4">
            暂无评论，快来抢沙发吧~
        </n-alert>

        <n-card v-for="comment in comments" :key="comment.comment_id">
            <n-p>{{ comment.content }}</n-p>
            <n-flex inline :wrap="false" justify="start" align="center">
                <n-text type="success"
                    >来自: {{ comment.user }} | {{ comment.created_at }}</n-text
                >

                <n-button @click="respondComment(comment.user)"> 回复</n-button>

                <n-button
                    v-if="userStore.identity == 'admin'"
                    @click="handlerDeleteComment(comment.comment_id)"
                >
                    删除</n-button
                >
            </n-flex>
        </n-card>
    </n-layout>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import {
    NInput,
    NLayout,
    useDialog,
    NFlex,
    NFormItem,
    NForm,
    NButton,
    NCard,
    useMessage,
    type FormInst,
    NAlert,
    NH2,
    NText,
    NP,
} from "naive-ui";
import { computed, inject, onMounted, onUnmounted, ref, type Ref } from "vue";
import { DeleteComment, fetchComments, postComment } from "@/api/comment";
import { useRoute, useRouter } from "vue-router";
import EntryCommentBar from "./EntryCommentBar.vue";

const commentTitle = ref<HTMLElement | null>(null);
const isFixed = ref(false);
const message = useMessage();
const userStore = useUserStore();
const route = useRoute();
const articleId = route.params.id as string;
const comments = ref<any[]>([]);
const ifComment = ref(false);
const dialog = useDialog();

// 初始化
const loadComments = async () => {
    const res = await fetchComments(articleId);
    comments.value = res.data.comments;
    ifComment.value = comments.value.length > 0;
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
const respondComment = (username: string) => {
    // todo 因为分了模块，这里后期再具体实现
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
</script>

<style scoped>
/* 优化这个css使其风格接近chatgpt的输入框 */
.fixedBottom {
    position: fixed;
    bottom: 0;
    left: 250px;
    right: 0;
    background: #fff;
    z-index: 1000;
}
</style>

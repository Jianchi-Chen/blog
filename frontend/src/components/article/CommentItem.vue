<template>
    <div>
        <div v-if="!isChild">
            <n-p>{{ comment.content }}</n-p>
            <n-flex inline :wrap="false" justify="start" align="center">
                <n-button
                    @click="$emit('like', comment.comment_id, 'main')"
                    :bordered="false"
                >
                    <div v-if="comment.liked_by_me === 0">
                        <n-icon size="20"><FavoriteBorderOutlined /></n-icon>
                    </div>
                    <div v-else>
                        <n-icon size="20"><FavoriteOutlined /></n-icon>
                    </div>
                    {{ comment.like_count }}
                </n-button>

                <n-text type="success"
                    >来自: {{ comment.user }} | {{ comment.created_at }}</n-text
                >

                <n-button
                    @click="$emit('reply', comment.user, comment.comment_id)"
                    >回复</n-button
                >

                <n-button
                    v-if="isAdmin"
                    @click="$emit('delete', comment.comment_id)"
                    >删除</n-button
                >
            </n-flex>
        </div>

        <div
            v-for="child in comment.children"
            :key="child.comment_id"
            class="child-comment"
        >
            <n-p>{{ child.content }}</n-p>

            <n-flex inline :wrap="false" justify="start" align="center">
                <n-button
                    @click="$emit('like', child.comment_id, 'child')"
                    :bordered="false"
                >
                    <div v-if="child.liked_by_me === 0">
                        <n-icon size="20"><FavoriteBorderOutlined /></n-icon>
                    </div>
                    <div v-else>
                        <n-icon size="20"><FavoriteOutlined /></n-icon>
                    </div>
                    {{ child.like_count }}
                </n-button>

                <n-text type="success"
                    >来自: {{ child.user }} | {{ child.created_at }}</n-text
                >

                <n-button
                    @click="$emit('reply', child.user, comment.comment_id)"
                    >回复</n-button
                >

                <n-button
                    v-if="isAdmin"
                    @click="$emit('delete', child.comment_id)"
                    >删除</n-button
                >
            </n-flex>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import { NButton, NFlex, NText, NIcon, NP } from "naive-ui";
import { FavoriteBorderOutlined, FavoriteOutlined } from "@vicons/material";

const props = defineProps<{ comment: any; isChild?: boolean }>();
const emits = defineEmits(["like", "reply", "delete"] as const);

const userStore = useUserStore();
const isAdmin = userStore.identity === "admin";
</script>

<style scoped>
.child-comment {
    margin-top: 12px;
    margin-bottom: 12px;
    margin-left: 32px;
    padding: 12px 16px;
    border-left: 3px solid var(--n-border-color, #e0e0e0);
    border-radius: 2px;
    background: var(--n-color, #fafafa);
}
</style>

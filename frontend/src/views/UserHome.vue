<template>
    <div class="user-home">
        <n-grid :x-gap="24" :cols="3" :cols-sm="1">
            <!-- 侧栏：头像 + 签名 -->
            <n-grid-item :span="1">
                <UserProfile :user="user" />
            </n-grid-item>

            <!-- 主要：收藏文章列表 -->
            <n-grid-item :span="2">
                <FavoriteArticles />
            </n-grid-item>
        </n-grid>
    </div>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import UserProfile from "@/components/user/UserProfile.vue";
import FavoriteArticles from "@/components/user/FavoriteArticles.vue";
import { reactive } from "vue";

const userstore = useUserStore();

// 用户基础信息
const user = reactive({
    id: userstore.id || "未发现ID",
    name: userstore.username || "未发现用户名",
});
</script>

<style scoped>
.user-home {
    padding: 20px;
}
</style>

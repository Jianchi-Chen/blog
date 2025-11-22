<template>
    <n-flex justify="space-between" align="center">
        <n-flex align="center">
            <n-button
                :focusable="false"
                style="font-size: 20px"
                :text="false"
                @click="handlerFolderExpand"
            >
                <n-icon size="27">
                    <FolderOpen />
                </n-icon>
            </n-button>
            <n-switch @click="submitToggleTheme">
                <template #checked> Dark </template>
                <template #unchecked> Light </template>
            </n-switch>
            <!-- 把搜索组件放在菜单最右侧 -->
            <NavSearch @search="onSearch" />
        </n-flex>

        <n-flex>
            <n-menu
                :options="menuOptions"
                v-model:value="activeKey"
                mode="horizontal"
                @update:value="handleSelect"
                responsive
            />
        </n-flex>
    </n-flex>
</template>

<script setup lang="ts">
import { useMessage } from "naive-ui";
import type { MenuOption } from "naive-ui";
import { NMenu } from "naive-ui";
import { useUserStore } from "@/stores/user";
import { computed, h, ref } from "vue";
import { useRouter } from "vue-router";
import NavSearch from "@/components/NavSearch.vue";
import { FolderOpen } from "@vicons/ionicons5";
import { useArticleStore } from "@/stores/article";

const userstore = useUserStore();
const router = useRouter();
const message = useMessage();
const articleStore = useArticleStore();

// !!user.token 是布尔值，专门用来表示“是否登录”这个状态
const isLoggedin = computed(() => !!userstore.token);
const activeKey = ref();
const emit = defineEmits<{
    toggleTheme: [];
}>();

const submitToggleTheme = () => {
    // console.log("theme change");
    emit("toggleTheme");
};

/** 收到子组件的搜索关键词后跳转或过滤 */
function onSearch(kw: string) {
    // 示例：跳转到搜索结果页
    // router.push({ name: 'ArticleList', query: { q: kw } })
    message.success("搜索中...");
}

// 利用computed(), 来创建基于其他响应式数据的派生值。当依赖变化时, 其会重新计算
const menuOptions = computed(() => {
    const items = [
        {
            label: "博客",
            key: "/",
        },
    ];

    // 基于登录状态, 显示导航内容
    if (isLoggedin.value && userstore.isAdmin()) {
        items.push({ label: "后台管理", key: "/admin" });
        items.push({ label: "退出", key: "logout" });
        items.push({
            label: `当前用户: ${userstore.username}`,
            key: "username",
        });
    } else if (isLoggedin.value) {
        items.push({ label: "退出", key: "logout" });
        items.push({
            label: `当前用户: ${userstore.username}`,
            key: "username",
        });
    } else {
        items.push({ label: "登录", key: "/login" });
    }

    return items;
});
const handleSelect = (key: string) => {
    if (key == "logout") {
        userstore.logout();
        message.success("退出成功");
        router.push("/");
    } else {
        activeKey.value = key;
        router.push(`${key}`);
    }
};

// 控制侧边导航栏展开与否
const handlerFolderExpand = () => {
    articleStore.expandFolder = !articleStore.expandFolder;
};
</script>

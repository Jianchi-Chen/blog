<template>
    <n-menu :options="menuOptions" v-model:value="activeKey" mode="horizontal" @update:value="handleSelect"
        responsive />
</template>

<script setup lang="ts">
import { useMessage } from "naive-ui";
import type { MenuOption } from 'naive-ui'
import { useUserStore } from "@/stores/user";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

const userstore = useUserStore();
const router = useRouter();
const message = useMessage();

// !!user.token 是布尔值，专门用来表示“是否登录”这个状态
const isLoggedin = computed(() => !!userstore.token);

const activeKey = ref();

// 利用computed(), 来创建基于其他响应式数据的派生值。当依赖变化时, 其会重新计算
const menuOptions = computed(() => {
    const items = [
        {
            label: "博客",
            key: "/",
        }
    ]

    // 基于登录状态, 显示导航内容
    if (isLoggedin.value && userstore.isAdmin()) {
        items.push({ label: '后台管理', key: '/admin' })
        items.push({ label: '退出', key: 'logout' })
    } else if (isLoggedin.value) {
        items.push({ label: '退出', key: 'logout' })
    } else {
        items.push({ label: '登录', key: '/login' })
    }

    return items;
})
const handleSelect = (key: string) => {
    if (key == 'logout') {
        userstore.logout();
        message.success('退出成功');
        router.push('/');
    } else {
        activeKey.value = key;
        router.push(`${key}`);
    }

};
</script>

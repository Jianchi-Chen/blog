<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import NavBar from "./components/NavBar.vue";
import {
    darkTheme,
    lightTheme,
    NConfigProvider,
    NLayout,
    NLayoutHeader,
    NLayoutContent,
    NLayoutFooter,
    NLayoutSider,
} from "naive-ui";
import type { GlobalThemeOverrides, GlobalTheme } from "naive-ui";
import { useUserStore } from "./stores/user";
import { computed, onMounted, provide, ref } from "vue";
import Sider from "@/components/Sider.vue";
import { useArticleStore } from "./stores/article";
import { BackToTop } from "@vicons/carbon";

// 获取登录状态
const userStore = useUserStore();
const articleStore = useArticleStore();
onMounted(() => {
    userStore.initFromStorage();
});

// 主题色切换, null 等于 light
const theme = ref<GlobalTheme | null>(null);
const toggleTheme = () => {
    theme.value = articleStore.osTheme === false ? darkTheme : null;
    articleStore.osTheme = !articleStore.osTheme;
};
</script>

<template>
    <!-- 整个 Naive UI 的全局配置上下文,例如主题、语言、图标等 -->
    <n-config-provider :theme="theme">
        <!-- 模态框 -->
        <n-modal-provider>
            <!-- 对话框 -->
            <n-dialog-provider>
                <!-- 所有页面组件都能访问 useMessage() 提供的 API -->
                <n-message-provider>
                    <n-layout position="absolute" class="h-screen h-full">
                        <n-layout-header bordered>
                            <NavBar @toggleTheme="toggleTheme" />
                        </n-layout-header>

                        <!-- 百分之94高, 是除开NavBar组件的高度 -->
                        <n-layout has-sider class="h-[94%]">
                            <!-- 天坑：不加collapse-mode="width"的话图标无法展示 -->
                            <n-layout-sider
                                :collapsed="articleStore.expandFolder"
                                :width="240"
                                :collapsed-width="64"
                                :size="100"
                                @collapse="articleStore.expandFolder = true"
                                @expand="articleStore.expandFolder = false"
                                collapse-mode="width"
                                class="!mr-2"
                                bordered
                            >
                                <Sider />
                            </n-layout-sider>

                            <n-layout-content>
                                <router-view />

                                <n-back-top
                                    :bottom="140"
                                    :visibility-height="300"
                                >
                                    <n-icon-wrapper
                                        :size="45"
                                        :border-radius="18"
                                    >
                                        <n-icon size="40">
                                            <BackToTop />
                                        </n-icon>
                                    </n-icon-wrapper>
                                </n-back-top>
                            </n-layout-content>
                        </n-layout>
                    </n-layout>
                </n-message-provider>
            </n-dialog-provider>
        </n-modal-provider>
    </n-config-provider>
</template>

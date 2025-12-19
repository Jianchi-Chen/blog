<script setup lang="ts">
import { RouterView } from "vue-router";
import NavBar from "./components/NavBar.vue";
import {
    darkTheme,
    NConfigProvider,
    NLayout,
    NLayoutHeader,
    NLayoutContent,
    NLayoutSider,
} from "naive-ui";
import type { GlobalTheme } from "naive-ui";
import { useUserStore } from "./stores/user";
import { onMounted, ref } from "vue";
import Sider from "@/components/Sider.vue";
import { useArticleStore } from "./stores/article";
import { BackToTop } from "@vicons/carbon";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { ask } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "@/stores/app";

// 获取登录状态
const userStore = useUserStore();
const appStore = useAppStore();
const articleStore = useArticleStore();
onMounted(() => {
    if (appStore.isTauri) {
        autoUpdate();
    }
    userStore.initFromStorage();
});

// 主题色切换, null 等于 light
const theme = ref<GlobalTheme | null>(null);
const toggleTheme = () => {
    theme.value = articleStore.osTheme === false ? darkTheme : null;
    articleStore.osTheme = !articleStore.osTheme;
};

// 检查更新
const autoUpdate = async () => {
    try {
        const update = await check(); // 访问 endpoint
        if (!update) return;

        const yes = await ask(
            `发现新版本 ${update.version}\n${update.body}\n\n是否立即下载并重启?`,
            { title: "更新提示" }
        );
        if (!yes) return;
        await update.downloadAndInstall(() => {
            // 可选：在此处把进度 event 画到 UI，而不是输出到控制台
            console.log(event);
        });
        try {
            await relaunch(); // 重启即生效
        } catch (error) {
            console.error(
                "Failed to relaunch application after update:",
                error
            );
            try {
                await ask(
                    "应用已更新，但重启失败。请手动重启应用以完成更新。",
                    { title: "重启失败" }
                );
            } catch (dialogError) {
                console.error(
                    "Failed to show restart failure dialog:",
                    dialogError
                );
            }
        }
        await relaunch(); // 重启即生效
    } catch (error) {
        console.error("自动更新检查或安装失败:", error);
        await ask("检查更新时发生错误，请稍后重试。", { title: "更新失败" });
    }
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

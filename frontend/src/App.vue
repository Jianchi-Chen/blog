<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue';
import { darkTheme, lightTheme, NConfigProvider, NLayout, NLayoutHeader, NLayoutContent, NLayoutFooter, NLayoutSider } from 'naive-ui'
import type { GlobalThemeOverrides, GlobalTheme } from 'naive-ui';
import { useUserStore } from './stores/user';
import { onMounted, ref } from 'vue';
import Sider from '@/components/Sider.vue'
import { useArticleStore } from './stores/article';

// 获取登录状态
const userStore = useUserStore();
const articleStore = useArticleStore();
onMounted(() => {
  userStore.initFromStorage();
})

// 主题色切换, null 等于 light
const theme = ref<GlobalTheme | null>(null)
const toggleTheme = () => {
  // console.log("app.vue theme change");
  // console.log(theme.value);
  theme.value = articleStore.osTheme === false ? darkTheme : null
  articleStore.osTheme = !articleStore.osTheme
}

</script>

<template>
  <!-- 整个 Naive UI 的全局配置上下文，例如主题、语言、图标等 -->
  <n-config-provider :theme="theme" class="w-screen h-screen m-0 p-0">
    <!-- 对话框使用 -->
    <n-dialog-provider>
      <!-- 所有页面组件都能访问 useMessage() 提供的 API -->
      <n-message-provider>

        <n-layout>

          <n-layout-header class="fixed top-0 left-0 right-0 z-50 bg-white shadow-md" bordered>
            <!-- <NavBar /> 是顶级组件，它会一直显示在页面上，无论用户访问哪个路径。-->
            <NavBar @toggleTheme="toggleTheme" />
          </n-layout-header>

          <n-layout has-sider>
            <!-- 天坑：不加collapse-mode="width"的话图标无法展示 -->
            <n-layout-sider :collapsed="articleStore.expandFolder" :width="240" :collapsed-width="64"
              @collapse="articleStore.expandFolder = true" @expand="articleStore.expandFolder = false"
              collapse-mode="width" class="sticky top-[64px] " bordered>

              <Sider />
            </n-layout-sider>

            <n-layout-content class="h-screen pt-[64px] pl-5">
              <!-- <router-view /> 是vue-router 提供的占位组件，表示“当前路由匹配到的页面组件”应该渲染在这里。-->
              <router-view />
            </n-layout-content>
          </n-layout>

        </n-layout>

        <template>
          <n-back-top :right="100" />
        </template>
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue';
import { darkTheme, lightTheme, NConfigProvider, NLayout, NLayoutHeader, NLayoutContent, NLayoutFooter, NLayoutSider } from 'naive-ui'
import type { GlobalThemeOverrides, GlobalTheme } from 'naive-ui';
import { useUserStore } from './stores/user';
import { onMounted } from 'vue';

// 获取登录状态
const userStore = useUserStore();
onMounted(() => {
  userStore.initFromStorage();
})

</script>

<template>
  <div class="bg-[#fdfdfd] text-[#1a1a1a] min-h-screen min-w-screen">
    <!-- <NavBar /> 是顶级组件，它会一直显示在页面上，无论用户访问哪个路径。-->

    <!-- 整个 Naive UI 的全局配置上下文，例如主题、语言、图标等 -->
    <n-config-provider>
      <!-- 对话框使用 -->
      <n-dialog-provider>
        <!-- 所有页面组件都能访问 useMessage() 提供的 API -->
        <n-message-provider>

          <n-layout>

            <n-layout-header>
              <NavBar />
            </n-layout-header>

            <n-layout has-sider>
              <n-layout-sider>
                <Sider />
              </n-layout-sider>

              <n-layout-content>
                <!-- <router-view /> 是vue-router 提供的占位组件，表示“当前路由匹配到的页面组件”应该渲染在这里。-->
                <router-view />
              </n-layout-content>

            </n-layout>

          </n-layout>

        </n-message-provider>
      </n-dialog-provider>
    </n-config-provider>
  </div>

</template>

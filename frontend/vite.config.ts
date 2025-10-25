import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
import Components from "unplugin-vue-components/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    tailwindcss(),

    // 1. 自动引入组件
    Components({
      resolvers: [NaiveUiResolver()], // 关键
      dts: true, // 生成 .d.ts，TS 有提示
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },

  // 配置 Vite 代理
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:5173", // ← 这里填你的后端端口
        changeOrigin: true,
        rewrite: (path) => path,
      },
    },
  },

  // 避免 Vite 在构建时错误地处理 Naive UI 的某些模块。
  optimizeDeps: {
    exclude: ["naive-ui"],
  },
  build: {
    commonjsOptions: {
      include: [/node_modules/],
    },
  },
});

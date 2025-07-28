import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
// import Components from "unplugin-vue-components/vite";
// import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
// import AutoImport from "unplugin-auto-import/vite";
// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    tailwindcss(),
    
    // 弃用, 自动导入会导致vscode无法给出提示
    // unplugin-vue-components, 常与下文中的auto-import配合使用
    // Components({ resolvers: [NaiveUiResolver()] }),

    // 弃用, 自动导入会导致vscode无法给出提示
    // unplugin-auto-import
    // AutoImport({
    //   imports: [
    //     "vue", // 自动导入 ref, reactive 等 Vue API
    //     {
    //       "naive-ui": [
    //         "useMessage",
    //         "useNotification",
    //         "useDialog",
    //         "useLoadingBar",
    //         "createDiscreteApi",
    //       ],
    //     },
    //   ],
    //   dts: "src/auto-imports.d.ts",
    // }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});

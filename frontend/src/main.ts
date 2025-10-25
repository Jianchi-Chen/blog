import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import naive from "naive-ui";
// markdown编辑器
import 'vditor/dist/index.css'
// 通用字体
import "vfonts/Lato.css";

// msw
// if (import.meta.env.DEV) {
//   const { worker } = await import("./mocks/browser.ts");
//   await worker.start({
//     onUnhandledRequest: "bypass", // 未 mock 的请求直接放行
//   });
// }

const app = createApp(App);

// Piania、Router、Naive-ui
app.use(createPinia());
app.use(router);
app.use(naive);

app.mount("#app");


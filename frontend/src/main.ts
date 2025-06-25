import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";



// msw
if (import.meta.env.DEV) {
  const { worker } = await import("./mocks/browser.ts");
  await worker.start({
    onUnhandledRequest: "bypass", // 未 mock 的请求直接放行
  });
}

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount("#app");

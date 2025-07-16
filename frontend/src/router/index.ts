import { useUserStore } from "@/stores/user";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("@/views/Home.vue"),
    },
    {
      path: "/article/:id",
      name: "ArticleDetail",
      component: () => import("@/views/ArticleDetail.vue"),
    },
    {
      path: "/login",
      name: "Login",
      component: () => import("@/views/Login.vue"),
    },
    {
      path: "/admin",
      name: "Admin",
      meta: { requiresLogin: true },
      component: () => import("@/views/Admin.vue"),
    },
    {
      path: "/admin/create",
      name: "AdminCreate",
      meta: { requiresAuth: true },
      component: () => import("@/views/AdminCreate.vue"),
    },
  ],
});

// 全局前置守卫：判断是否登录
// to：即将进入的目标路由对象
// from：当前导航正要离开的路由对象
// next()：必须调用，用来决定导航行为
router.beforeEach((to, _, next) => {
  // 如果不加()，则user本身是一个函数引用，并不会拿到useUserStore中的值
  const user = useUserStore();

  // 即将进入的目标对象具有requireLogin属性，并且user.token里没有任何内容的话，送去/login
  if (to.meta.requireLogin && !user.token) {
    next("/login");
  } else {
    next(); // 放行
  }
});

export default router;

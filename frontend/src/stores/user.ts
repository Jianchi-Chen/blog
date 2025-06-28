import { defineStore } from "pinia";

export const useUserStore = defineStore("user", {
  // state 本身是一个函数，调用useUserStore时会被初始化执行一次
  // 箭头函数的值返回给了pinia store的内部状态系统，成为这个store的默认数据源
  state: () => ({
    token: "", // 存储 JWT token
    username: "", // 当前登录用户
  }),
  actions: {
    login(token: string, username: string) {
      this.token = token;
      this.username = username;
      localStorage.setItem("token", token);
      localStorage.setItem("username", username);
    },

    logout() {
      this.token = "";
      this.username = "";
      localStorage.removeItem("token");
      localStorage.removeItem("username");
    },

    initFromStorage() {
      this.token = localStorage.getItem("token") || "";
      this.username = localStorage.getItem("username") || "";
    },

    // 判断当前用户是否是管理员
    // TODO 可能存在问题
    isAdmin() {
      if (this.username == "admin") {
        return true;
      }
      return false;
    },
  },
});

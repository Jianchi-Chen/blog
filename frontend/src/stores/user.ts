import type { User } from "@/types/user";
import { defineStore } from "pinia";
import { uuidv7 } from "uuidv7";

export const useUserStore = defineStore("user", {
  // state 本身是一个函数，调用useUserStore时会被初始化执行一次
  // 箭头函数的值返回给了pinia store的内部状态系统，成为这个store的默认数据源
  state: (): User => ({
    id: "",
    token: "", // 存储 JWT token
    username: "", // 当前登录用户
    password: "",
    identity: "vistor", // 当前用户身份
  }),

  // 全局行为函数
  actions: {
    login(token: string, username: string, id?: string) {
      this.token = token;
      this.username = username;
      localStorage.setItem("token", token);
      localStorage.setItem("username", username);
      // localStorage.setItem("id", id);
    },

    logout() {
      this.token = "";
      this.username = "";
      this.id = "";
      localStorage.removeItem("token");
      localStorage.removeItem("username");
    },

    initFromStorage() {
      this.token = localStorage.getItem("token") || "";
      this.username = localStorage.getItem("username") || "";
    },

    // 判断当前用户是否是管理员
    isAdmin() {
      if (this.username === "admin" || this.username === "123") {
        this.identity = "admin";
        return true;
      }
      return false;
    },
  },
});

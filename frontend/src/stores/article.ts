import type { Article } from "@/types/article";
import { defineStore } from "pinia";
import type { Ref } from "vue";

export const useArticleStore = defineStore("article", {
  state: () => ({
    updateFolderContentSignal: false,
    expandFolder: false,
    osTheme: false,
  }),

  actions: {
    setUpdateFolderContentSignal(value: boolean) {
      this.updateFolderContentSignal = value;
    },
  },
});

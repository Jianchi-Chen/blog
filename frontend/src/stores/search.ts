import type { Article } from "@/types/article";
import { defineStore } from "pinia";

export const useSearchStore = defineStore("search", {
  state: () => ({
    condition: "",
  }),

  actions: {
    setCondition(value: string) {
      this.condition = value;
    },
  },
});

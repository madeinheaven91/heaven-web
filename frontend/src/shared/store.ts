import { defineStore } from "pinia";
import { type User } from "@/entities/user";

export const useAuthStore = defineStore("auth", {
  state: () => ({
    user: null as User | null,
  }),

  getters: {
    isAuthenticated: (state) => state.user !== null,
    getUser: (state) => state.user,
  },

  actions: {
    login(user: User | null) {
      this.user = user;
    },
    logout() {
      this.user = null;
    }
  }
});

import { defineStore } from "pinia";
import { User } from "./models";

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

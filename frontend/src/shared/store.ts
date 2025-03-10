import { createStore } from "vuex"
import type { User } from "./models"

const store = createStore({
  state () {
    return {
      user: null as User | null
    }
  },
  mutations: {
    login (state: any, user: User) {
      state.user = user;
    },
    logout (state: any) {
      state.user = null;
    }
  },
  getters: {
    isAuthenticated: (state: any) => state.user !== null,
    getUser: (state: any) => state.user
  }
})

export default store;

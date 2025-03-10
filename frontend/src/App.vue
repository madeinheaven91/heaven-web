<script setup lang="ts">
import Navbar from './components/Navbar.vue'
import PostList from './components/PostList.vue'
import { RouterLink, RouterView }  from 'vue-router'
import { ref, onMounted, reactive, computed } from 'vue'
import { login, load_profile, remove_token } from './shared/utils.ts'
import { type User } from './shared/models.ts'
import { useStore } from 'vuex'

const store = useStore();

onMounted(async () => {
  let profile = await load_profile();
  store.commit('login', profile);
})

</script>

<template>
  <header>
    <Navbar />
  </header>
  <div class="container py-3">
    <RouterView />
  </div>
</template>

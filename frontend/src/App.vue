<script setup lang="ts">
import Navbar from './components/Navbar.vue'
import {  RouterView }  from 'vue-router'
import { load_profile } from './shared/utils.ts'
import { onMounted } from 'vue'
import { useAuthStore } from './shared/store.ts'

const store = useAuthStore();

onMounted(async () => {
  let profile = await load_profile();
  store.login(profile);
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

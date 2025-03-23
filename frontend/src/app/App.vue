<script setup lang="ts">
import Navbar from '@/widgets/Navbar.vue'
import { UserApi } from '@/entities/user'
import { RouterView }  from 'vue-router'
import { onMounted } from 'vue'
import { useAuthStore } from '@/shared/store.ts'

const store = useAuthStore();

onMounted(async () => {
  let profile = await UserApi.load_profile();
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

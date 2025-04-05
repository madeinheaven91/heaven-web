<script setup lang="ts">
import { RouterView } from 'vue-router'
import { UserApi } from '@/entities/user'
import { onMounted } from 'vue'
import { useAuthStore } from '@/shared/store.ts'

const store = useAuthStore();

onMounted(async () => {
  let profile = await UserApi.load_profile();
  store.login(profile);
})

</script>

<template>
  <RouterView />
</template>

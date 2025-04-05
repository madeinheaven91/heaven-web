<script setup lang="ts">
import Button from '@/shared/ui/button'
import Logo from '@/shared/ui/logo'
import Modal from '@/shared/ui/modal'
import LoginForm from '@/widgets/login-form'
import { RouterLink } from "vue-router";
import { useAuthStore } from '@/shared/store.ts'
import { ref, computed } from 'vue'

const modalRef = ref<InstanceType<typeof Modal> | null>(null);
const store = useAuthStore();
const user = computed(() => store.getUser);
const isAuthenticated = computed(() => store.isAuthenticated);
</script>

<template>
  <header class="">
    <ul class="px-10 items-center list">
      <!-- ROUTER -->
      <li><RouterLink to='/blog'>
        <Logo/>
      </RouterLink></li>
      <div v-if="isAuthenticated" class="list">
        <RouterLink to='/blog/drafts'>
          <Button>Черновики</Button>
        </RouterLink>
        <RouterLink to='/blog/new'>
          <Button>Новый пост</Button>
        </RouterLink>
        <Button @click="store.logout">Выход</Button>
        <p class="c-yellow">madeinheaven91</p>
      </div>
      <div v-else class="list">
        <Button @click="modalRef?.openModal()">Вход</Button>
        <Modal ref='modalRef'>
          <LoginForm :onClose="modalRef?.closeModal"/>
        </Modal>
      </div>
    </ul>
  <hr class="hr w-[90%]">
  <hr class="hr w-[85%]">
  </header>
</template>

<style lang="css" scoped>
a {
  text-decoration: none;
  text-shadow: none;
  color: var(--main);
}
.list {
  @apply flex justify-start flex-row items-center;
  gap: 1rem;
}
.hr{
  margin: 5px auto;
}
</style>

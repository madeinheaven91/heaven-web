<script setup lang="ts">
import Button from '@/shared/ui/button'
import Logo from '@/shared/ui/logo'
import Modal from '@/shared/ui/modal'
import LoginForm from '@/widgets/login-form'
import Sidebar from './Sidebar.vue'
import { RouterLink } from "vue-router";
import { useAuthStore } from '@/shared/store.ts'
import { ref, computed } from 'vue'

const store = useAuthStore();
const user = computed(() => store.getUser);
const isAuthenticated = computed(() => store.isAuthenticated);
const modalRef = ref<InstanceType<typeof Modal> | null>(null);

const sidebarIsOpen = ref(false)
</script>

<template>
  <!-- MOBILE -->
  <header class="block lg:hidden">
    <div class="flex justify-between items-center px-5 pt-2">
      <RouterLink to='/blog'>
        <Logo/>
      </RouterLink>
      <Sidebar v-if="sidebarIsOpen" @toggleOpen="sidebarIsOpen = false"/>
      <img class="filter-main cursor-pointer" @click="sidebarIsOpen = !sidebarIsOpen" width="32px" src="/public/icons/list.svg"/>
    </div>
    <hr class="hr mx-auto mt-1 w-[90%]">
    <hr class="hr mx-auto w-[85%]">
  </header>
  <!-- PC -->
  <header class="hidden lg:block">
    <ul class="px-10 items-center list">
      <!-- ROUTER -->
      <li><RouterLink to='/blog'>
        <Logo/>
      </RouterLink></li>
      <div v-if="isAuthenticated && user" class="list">
        <RouterLink to='/blog/drafts'>
          <Button>Черновики</Button>
        </RouterLink>
        <RouterLink to='/blog/new'>
          <Button>Новый пост</Button>
        </RouterLink>
        <Button @click="store.logout">Выход</Button>
        <p class="c-yellow">{{ user.name }}</p>
      </div>
      <div v-else class="list">
        <Button @click="modalRef?.openModal()">Вход</Button>
        <Modal ref='modalRef'>
          <LoginForm :onClose="() => modalRef?.closeModal()"/>
        </Modal>
      </div>
    </ul>
    <hr class="hr mx-auto w-[90%]">
    <hr class="hr mx-auto w-[85%]">
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
</style>

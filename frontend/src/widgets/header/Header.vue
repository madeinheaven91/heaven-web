<script setup lang="ts">
import Button from '@/shared/ui/button'
import Logo from '@/shared/ui/logo'
import Modal from '@/shared/ui/modal'
import LoginForm from '@/widgets/login-form'
import { ref } from 'vue'

const modalRef = ref<InstanceType<typeof Modal> | null>(null);
const isAuthenticated = false;
</script>

<template>
  <header class="">
    <ul class="px-10 items-center list">
      <!-- ROUTER -->
      <li><Logo/></li> 
      <div v-if="isAuthenticated" class="list">
        <Button>Черновики</Button>
        <Button>Новый пост</Button>
        <Button>Выход</Button>
        <Button>madeinheaven91</Button>
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
.list {
  @apply flex justify-start flex-row;
  gap: 1rem;
}
.hr{
  margin: 5px auto;
}
</style>

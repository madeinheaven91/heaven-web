<script setup lang="ts">
import Button from "@/shared/ui/button";
import { useAuthStore } from "@/shared/store.ts";
import { UserApi } from "@/entities/user";
import { ref } from "vue";
import { useRouter } from 'vue-router'

const router = useRouter();

const username = ref("");
const password = ref("");
const errorMessage = ref("");
const store = useAuthStore();

const props = defineProps({
  onClose: {
    type: Function,
    required: true,
  },
});

const onSubmit = async () => {
  try {
    errorMessage.value = "";
    const response = await UserApi.login(username.value, password.value);
    store.login(response.user);
    localStorage.setItem("access_token", response.access_token);
    router.go(0);
  } catch (error) {
    errorMessage.value = "Неправильный логин или пароль!";
  }
};
</script>

<template>
  <div class="flex flex-col gap-3">
    <h3 class='text-center'>Авторизация</h3>
    <p v-if="errorMessage" style="color: var(--red)">{{ errorMessage }}</p>
    <form class="flex flex-col gap-3" @submit.prevent="onSubmit">
      <div class="flex gap-5">
        <div class="flex flex-col gap-1">
          <label for="usernameInput">Имя</label>
          <label for="passwordInput">Пароль</label>
        </div>
        <div class="flex flex-col gap-1">
          <input
            type="text"
            id="usernameInput"
            placeholder="Дмитрий Прудников"
            class="input-field"
            v-model="username"
          />
          <input
            type="password"
            id="passwordInput"
            placeholder="12345"
            class="input-field"
            v-model="password"
          />
        </div>
      </div>
      <div class="flex justify-between px-10">
        <Button>
          <input type="submit" value="Войти" />
        </Button>
        <Button @click="() => props.onClose">Закрыть</Button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.input-field {
  background-color: var(--bg);
  color: var(--main);
  border-bottom-width: 2px;
  border-bottom-color: var(--main);
}
.input-field:focus{
  outline-style: solid;
  outline-width: 1px;
  outline-color: var(--yellow);
}
</style>

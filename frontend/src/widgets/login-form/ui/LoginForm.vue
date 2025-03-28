<script setup lang="ts">
import { ref } from "vue";
import { UserApi } from "@/entities/user";
import { useAuthStore } from "@/shared/store.ts";
import Button from "@/shared/ui/button";

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
    window.location.reload();
  } catch (error) {
    errorMessage.value = "Ошибка входа!";
  }
};
</script>

<template>
  <div class="flex flex-col gap-3">
    <h3>Авторизация</h3>
    <p v-if="errorMessage" style="color: var(--red)">{{ errorMessage }}</p>
    <form class="flex flex-col gap-3" @submit.prevent="onSubmit">
      <div class="flex gap-5">
        <div class="flex flex-col">
          <label for="usernameInput">Имя</label>
          <label for="passwordInput">Пароль</label>
        </div>
        <div class="flex flex-col">
          <input
            type="text"
            id="usernameInput"
            placeholder="Дмитрий Прудников"
            v-model="username"
          />
          <input
            type="password"
            id="passwordInput"
            placeholder="12345"
            v-model="password"
          />
        </div>
      </div>
      <div class="flex justify-between px-10">
        <Button>
          <input type="submit" value="Войти" />
        </Button>
        <Button @click="props.onClose">Закрыть</Button>
      </div>
    </form>
  </div>
</template>

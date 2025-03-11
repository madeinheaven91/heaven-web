<script setup lang="ts">
import { ref } from 'vue';
import { login } from '../shared/utils.ts';
import { useAuthStore } from '../shared/store.ts'

// Reactive state
const username = ref('');
const password = ref('');
const errorMessage = ref('');
const store = useAuthStore();

// Submit function
const onSubmit = async () => {
  try {
    errorMessage.value = '';
    const response = await login(username.value, password.value);

    store.login(response.user);
    localStorage.setItem("access_token", response.access_token);
    
    // Refresh page if required
    window.location.reload();
  } catch (error) {
    errorMessage.value = 'Ошибка входа!';
  }
};
</script>

<template>
  <div>
    <p v-if="errorMessage" class="text-danger">{{ errorMessage }}</p>
    
    <form @submit.prevent="onSubmit">
      <div class="form-group">
        <label for="usernameInput">Имя</label>
        <input 
          type="text" 
          class="form-control" 
          id="usernameInput" 
          placeholder="Дмитрий Прудников" 
          v-model="username"
        />
      </div>
      
      <div class="form-group mt-3">
        <label for="passwordInput">Пароль</label>
        <input 
          type="password" 
          class="form-control" 
          id="passwordInput" 
          placeholder="12345" 
          v-model="password"
        />
      </div>

      <div class="modal-footer mt-3 d-flex justify-content-around">
        <input type="submit" class="btn btn-primary" value="Войти" />
        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Закрыть</button>
      </div>
    </form>
  </div>
</template>

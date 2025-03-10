<script setup lang="ts">
import { ref, onMounted, reactive, computed } from 'vue'
import { login, load_profile, remove_token } from '../shared/utils.ts'
import type { User } from '../shared/models.ts'
import { useStore } from 'vuex'
import SmallModal from './SmallModal.vue'
import LoginForm from './LoginForm.vue'

const store = useStore();
const user = computed(() => store.state.user);
const isAuthenticated = computed(() => store.getters.isAuthenticated);

const logout = () => {
  store.commit('logout');
  remove_token();
}

const auth = async () => {
  await login('madeinheaven91', 'bebra');
  let profile = await load_profile();
  store.commit('login', profile);
}
</script>

<template>
  <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
    <div class="container-fluid">
      <a class="navbar-brand" href="/blog">JazzCoding</a> <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul v-if="isAuthenticated" class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <a class="nav-link" href="/blog/new">Добавить пост</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" href="/blog/drafts">Черновики</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" @click="logout">Выход</a>
          </li>
          <li class="nav-item">
            <a class="nav-link text-warning">{{user?.name}}</a>
          </li>
        </ul>
        <ul v-else class="navbar-nav me-auto mb-2 mb-lg-0">
          <!-- <li class="nav-item"> -->
          <!--   <a class="nav-link" @click="async () => { await auth()}">Вход</a> -->
          <!-- </li> -->
          <li class="nav-item">
              <a type="button" class="nav-link" data-bs-toggle="modal" data-bs-target="#login_modal">Вход</a>
              <SmallModal id="login_modal" title="Авторизация">
                <LoginForm />
              </SmallModal>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import Button from '@/shared/ui/button'
import Modal from '@/shared/ui/modal'
import LoginForm from '@/widgets/login-form'
import { RouterLink } from "vue-router";
import { useAuthStore } from '@/shared/store.ts'
import { ref, computed, onMounted, onUnmounted } from 'vue'

const modalRef = ref<InstanceType<typeof Modal> | null>(null);
const store = useAuthStore();
const user = computed(() => store.getUser);
const isAuthenticated = computed(() => store.isAuthenticated);

const isClosing = ref(false)
const loginOpen = ref(false)
const emit = defineEmits(['toggleOpen'])
// FIXME: when sidebar is opened, pressing escape will not toggle loginOpen and it will stay true
const toggleOpen = () => {
    if (!isClosing.value) {
        isClosing.value = true
        setTimeout(() => {
            emit('toggleOpen')
            isClosing.value = false
        }, 300);

    }
}
const toggleLogin = () => {
    loginOpen.value = !loginOpen.value
}
const handleKeydown = (event: KeyboardEvent) => {
    console.log(loginOpen.value)
    if (event.key === "Escape" && !loginOpen.value) {
        toggleOpen()
    }
};

onMounted(() => {
    window.addEventListener("keydown", handleKeydown);
});
onUnmounted(() => {
    window.removeEventListener("keydown", handleKeydown);
});
</script>

<template>
    <div class="fixed left-0 z-9 top-0 w-full h-full bg-black opacity-50"></div>
    <div class="sidebar p-5 fixed z-10 h-full border-l-2 text-2xl" :class="{ 'slide-out': isClosing }">
        <img class="filter-main cursor-pointer mb-5" @click="toggleOpen" width="32px" src="/public/icons/close.svg" />
        <!-- ROUTER -->
        <div v-if="isAuthenticated && user" class="list">
            <Button>
                <RouterLink to='/drafts'>Черновики</RouterLink>
            </Button>
            <Button>
                <RouterLink to='/new'>Новый пост</RouterLink>
            </Button>
            <Button @click="store.logout">Выход</Button>
            <p class="c-yellow">{{ user.name }}</p>
        </div>
        <div v-else class="list">
            <Button @click="() => { modalRef?.openModal(); toggleLogin() }">Вход</Button>
            <Modal ref='modalRef' class="text-base">
                <LoginForm :onClose="() => { modalRef?.closeModal(); toggleOpen() }" />
            </Modal>
        </div>
    </div>
</template>

<style scoped>
a {
    text-decoration: none;
    text-shadow: none;
    color: var(--main);
}

.list {
    @apply flex justify-start flex-col items-start;
    gap: 1rem;
}

.sidebar {
    top: 0;
    right: 0;
    background-color: var(--bg);
    animation: slide-in 0.3s;
}

.slide-out {
    animation: slide-out 0.3s forwards;
}

@keyframes slide-in {
    0% {
        transform: translateX(100%);
    }

    100% {
        transform: translateX(0);
    }
}

@keyframes slide-out {
    0% {
        transform: translateX(0);
    }

    100% {
        transform: translateX(100%);
    }
}
</style>

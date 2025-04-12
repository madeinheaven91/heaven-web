<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const emit = defineEmits(['toggleOpen'])
const isOpen = ref(false);

const openModal = () => {
  isOpen.value = true;
};

const closeModal = () => {
  isOpen.value = false;
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === "Escape") {
    closeModal();
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
});
onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
});

// Expose function globally if needed
defineExpose({ openModal, closeModal });
</script>

<template>
  <div v-if="isOpen" class="modal_wrapper">
    <div class="modal p-3">
      <slot/>
    </div>
  </div>
</template>

<style scoped>
.modal {
  position: fixed;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 10px;
  border-color: var(--main);
  border-width: 2px;
  border-style: double;
  z-index: 1200;
  background-color: var(--bg);
  font-size: 1em !important;
}

.modal_wrapper {
  position: fixed;
  background-color: rgba(0, 0, 0, 0.4);
  width: 100vw;
  height: 100vh;
  top: 0;
  left: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}
</style>

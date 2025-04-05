<script lang="ts" setup>
import Modal from "@/shared/ui/modal";
import Button from "@/shared/ui/button";
import { PostApi } from "@/entities/post";
import { ref } from "vue";

const props = defineProps({
  slug: String,
});

const modalRef = ref<InstanceType<typeof Modal> | null>(null);

const on_delete = async (slug: string) => {
  try {
    await PostApi.delete_post(slug);
    // FIXME: replace alerts with something nice
    alert("SUCCESS");
  } catch (error) {
    console.log(error);
  }
};
</script>

<template>
  <Button class="red-btn" @click="modalRef?.openModal()">Удалить</Button>
  <Modal ref="modalRef">
    <div class="d-flex mt-3 justify-content-between">
      <Button class="red-btn" @click="async () => await on_delete(props.slug as string)">Удалить</Button>
      <Button @click="modalRef?.closeModal()"> Отмена </Button>
    </div>
  </Modal>
</template>

<style scoped>
.red-btn {
  color: var(--red);
}

.red-btn:hover {
  border-bottom-color: var(--red);
}
</style>

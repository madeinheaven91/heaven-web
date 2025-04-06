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
    <div class="flex flex-col justify-between items-center">
      <h3>Удалить?</h3>
      <div class="flex justify-around mt-3">
        <Button class=" btn red-btn" @click="async () => await on_delete(props.slug as string)">Удалить</Button>
        <Button class="btn" @click="modalRef?.closeModal()"> Отмена </Button>
      </div>
    </div>
  </Modal>
</template>

<style scoped>
.btn {
  font-size: 20px;
}

.red-btn {
  color: var(--red);
}

.red-btn:hover {
  border-bottom-color: var(--red);
  cursor: pointer;
}
</style>

<script lang="ts" setup>
import SmallModal from './SmallModal.vue'
import { PostApi } from '@/entities/post'

const props = defineProps({
  slug: String
});

const on_delete = async (slug: string) => {
  try {
    await PostApi.delete_post(slug);
    alert("SUCCESS");
  }
  catch (error){
    console.log(error);
  }
};
</script>

<template>
  <a type="button" class="btn btn-danger" data-bs-toggle="modal" :data-bs-target="`#${props.slug}`">Удалить</a>
  <SmallModal title="Вы уверены?" :id="props.slug">
      <div class="d-flex mt-3 justify-content-between">
          <a class="btn btn-danger" @click="async () => await on_delete(props.slug as string)">Удалить</a>
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Отмена</button>
      </div>
  </SmallModal>
</template>

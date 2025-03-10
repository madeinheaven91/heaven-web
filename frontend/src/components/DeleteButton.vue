<script lang="ts" setup>
import { ref, onMounted, computed } from 'vue';
import TagList from './TagList.vue'
import SmallModal from './SmallModal.vue'
import { type User } from '../shared/models.ts'
import { useStore } from 'vuex'
import { delete_post } from '../shared/utils.ts'

const props = defineProps({
  slug: String
});

const store = useStore();
const user = computed(() => store.state.user);

const on_delete = async (slug) => {
  try {
    await delete_post(slug);
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
          <a class="btn btn-danger" @click="async () => await on_delete(props.slug)">Удалить</a>
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Отмена</button>
      </div>
  </SmallModal>
</template>

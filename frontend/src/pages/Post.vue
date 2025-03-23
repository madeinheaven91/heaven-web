<script setup lang="ts">
import { type Post, PostApi } from '@/entities/post';
import type { User } from '@/entities/user'
import { onMounted, ref, computed } from 'vue';
import { MdPreview, MdCatalog } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import { useAuthStore } from '@/shared/store.ts'
import DeleteButton from '@/widgets/DeleteButton.vue';

const store = useAuthStore();
const user = computed(() => store.user);

const props = defineProps({
  slug: String
});
const post = ref<Post | null>(null);
onMounted(async () => {
  post.value = await PostApi.fetch_post(props.slug as string);
})

</script>

<template>
  <div v-if="post">
    <div class="text-wrap text-break fs-1">{{post.title}}</div>
    <p class="text-wrap text-break text-secondary fs-3">{{post.author.name}}</p>
    <hr/>
    <MdPreview id="preview-only" :modelValue="post.body" class="font-imp" />
  </div>
  <div v-else>
    <h1>Пост не найден</h1>
  </div>
  <div class="d-flex gap-2 align-items-center">
    <button onclick="history.back()" class="btn btn-primary">Назад</button>
    <div v-if="user">
      <DeleteButton v-if="user.is_staff || user.id === post?.author.id" :slug="props.slug" />
    </div>
  </div>
</template>

<style>
.font-imp {
  font-family: 'Roboto Slab' !important;
}
</style>

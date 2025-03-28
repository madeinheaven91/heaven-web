<script setup lang="ts">
import Header from '@/widgets/header'
import { onMounted, ref } from 'vue'
import { type Post, PostApi} from '@/entities/post'
// import { type Tag, TagApi } from '@/entities/tag'
import { DateLib } from '@/shared/lib'

const props = defineProps({
  slug: String
});
const post = ref<Post | null>(null);
onMounted(async () => {
  if (!props.slug) return;
  let p = await PostApi.fetch_post(props.slug);
  // Object.assign(post, p);
  post.value = p;
})
</script>

<template>
  <Header/>
  <main v-if="post">
    <h1 class="break-all">{{ post.title }}</h1>
    <hr>
    <h4>{{ post.author.name }}</h4>
    <p>Создано: {{ DateLib.toLocale(post.created_at) }}</p>
    <p v-if="post.updated_at">Обновлено: {{ DateLib.toLocale(post.updated_at) }}</p>
    <hr>
    {{ post.body }}
  </main>
  <main v-else>
    <h1>Пост не найден</h1>
  </main>
</template>

<style lang="css" scoped>
p {
  margin: 0;
}
</style>

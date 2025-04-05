<script setup lang="ts">
import Button from '@/shared/ui/button'
import { type Post, PostApi} from '@/entities/post'
import { type Tag } from '@/entities/tag'
import Header from '@/widgets/header'
import TagList from '@/widgets/tag-list'
import { DateLib } from '@/shared/lib'
import { MdPreview } from 'md-editor-v3'
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter();

const props = defineProps({
  slug: String
});
const tags = ref<Tag[]>([]);
const post = ref<Post | null>(null);
onMounted(async () => {
  if (!props.slug) return;
  let p = await PostApi.fetch_post(props.slug);
  // Object.assign(post, p);
  post.value = p;
  tags.value = p.tags;
})
</script>

<template>
  <Header/>
  <main v-if="post">
    <h1 class="break-all mt-5">{{ post.title }}</h1>
    <TagList :tags="tags" />
    <hr>
    <h4>{{ post.author.name }}</h4>
    <p>Создано: {{ DateLib.toLocale(post.created_at) }}</p>
    <p v-if="post.updated_at">Обновлено: {{ DateLib.toLocale(post.updated_at) }}</p>
    <hr>
    <MdPreview v-model="post.body" language='en-US' preview-theme="heaven"/>
    <div class="my-5">
      <Button @click="router.push('/blog')">Назад</Button>
    </div>
  </main>
  <main v-else>
    <h1>Пост не найден</h1>
  </main>
</template>

<style lang="css" scoped>
p {
  margin: 0;
}

.md-editor-previewOnly {
  background-color: transparent;
}
</style>

<script setup lang="ts">
import Button from '@/shared/ui/button'
import DeleteButton from "@/shared/ui/delete-button";
import { type Post, PostApi} from '@/entities/post'
import { type Tag } from '@/entities/tag'
import { type User } from "@/entities/user";
import Header from '@/widgets/header'
import TagList from '@/widgets/tag-list'
import { DateLib } from '@/shared/lib'
import { MdPreview } from 'md-editor-v3'
import { onMounted, ref, computed } from 'vue'
import { useAuthStore } from "@/shared/store.ts";
import { useRouter } from 'vue-router'

const router = useRouter();
const store = useAuthStore();
const user = computed(() => store.user);

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
  <main class="px-5 lg:px-10">
    <template v-if="post">
      <h1 class="break-all mt-5 text-4xl">{{ post.title }}</h1>
      <TagList class="mt-3" :tags="tags" />
      <hr>
      <h4 class="text-3xl">{{ post.author.name }}</h4>
      <p class="my-0">Создано: {{ DateLib.toLocale(post.created_at) }}</p>
      <p class="my-0" v-if="post.updated_at">Обновлено: {{ DateLib.toLocale(post.updated_at) }}</p>
      <hr>
      <MdPreview v-model="post.body" language='en-US' preview-theme="heaven"/>
      <div class="my-5 flex gap-3">
        <Button @click="router.push('/blog')">Назад</Button>
        <template v-if="user">
          <Button v-if="(user as User).id == post.author.id">
            <RouterLink class="edit" :to="`/blog/post/${props.slug}/edit`">Редактировать</RouterLink>
          </Button>
          <DeleteButton v-if="(user as User).id == post.author.id || (user as User).is_staff" :slug="slug" />
        </template>
      </div>
    </template>
    <template v-else>
      <h1>Пост не найден</h1>
    </template>
  </main>
</template>

<style lang="css" scoped>
.md-editor-previewOnly {
  background-color: transparent;
}

.edit {
  text-decoration: none;
  color: var(--main);
  text-shadow: none;
  font-size: var(--f-m);
}

</style>

<script setup lang="ts">
import PostList from '@/widgets/PostList.vue'
import TagList from '@/widgets/TagList.vue'
import { onMounted, ref, computed } from 'vue'
import { TagApi, type Tag } from '@/entities/tag'
import { PostApi, type Post } from '@/entities/post'

let posts = ref<Post[]>([]);
let tags = ref<Tag[]>([]);

onMounted(async () => {
  let p = await PostApi.fetch_posts();
  let filtered = p.filter((ps: Post) => !ps.is_published);
  posts.value = filtered;
  let t = await TagApi.fetch_tags();
  tags.value = t;
})

</script>

<template>
  <h1>Посты</h1>
  <hr>
  <h2>Теги</h2>
  <TagList :tags="tags" fallback />
  <hr>
  <PostList :posts="posts" />
</template>

<style lang="css">

</style>

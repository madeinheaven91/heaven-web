<script setup lang="ts">
import PostList from '../components/PostList.vue'
import TagList from '../components/TagList.vue'
import { onMounted, ref, computed } from 'vue'
import type { Post, Tag } from '../shared/models.ts'
import { fetch_posts, fetch_tags } from '../shared/utils.ts'

let posts = ref<Post[]>([]);
let tags = ref<Tag[]>([]);

onMounted(async () => {
  let p = await fetch_posts();
  let filtered = p.filter(p => p.is_published);
  posts.value = filtered;
  let t = await fetch_tags();
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

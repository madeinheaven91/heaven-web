<script setup lang="ts">
import PostList from '@/widgets/PostList.vue'
import TagList from '@/widgets/TagList.vue'
import { onMounted, ref, computed } from 'vue'
import { type Post, PostApi } from '@/entities/post';
import { type Tag, TagApi } from '@/entities/tag';

let posts = ref<Post[]>([]);
let tags = ref<Tag[]>([]);

onMounted(async () => {
  let p = await PostApi.fetch_posts();
  let filtered = p.filter((ps: Post) => ps.is_published);
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

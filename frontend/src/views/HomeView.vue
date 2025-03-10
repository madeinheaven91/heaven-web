<script setup>
import PostList from '../components/PostList.vue'
import TagList from '../components/TagList.vue'
import { onMounted, ref, computed } from 'vue'
import { fetch_posts, fetch_tags } from '../shared/utils'

let posts = ref([]);
let tags = ref([]);

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

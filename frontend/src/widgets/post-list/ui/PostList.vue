<script setup lang="ts">
import { type Post } from '@/entities/post'
import PostListItem from './PostListItem.vue'
import { onMounted } from 'vue';

const props = defineProps({
  posts: Array<Post>
});
onMounted(() => {
})
</script>

<template>
  <div>
    <PostListItem v-if="props.posts !== null" v-for="post in props.posts" :post="post" />
    <h1 v-else>Постов нет :(</h1>
  </div>
</template>

<style lang="css" scoped>
h1 {
  opacity: 0.8;
}
.post {
  @apply border-2;
  padding-inline: 1rem;
  margin-bottom: 1rem;
}
</style>

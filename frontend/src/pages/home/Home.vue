<script setup lang="ts">
import Header from '@/widgets/header'
import PostList from '@/widgets/post-list'
import TagList from '@/widgets/tag-list'
import { type Post, PostApi } from '@/entities/post'
import { type Tag, TagApi } from '@/entities/tag'
import { onMounted, ref } from 'vue'

const posts = ref<Post[]>([]);
const tags = ref<Tag[]>([]);
onMounted(async () => {
  let ps = await PostApi.fetch_posts();
  posts.value = ps.filter((p: Post) => p.is_published);
  posts.value.sort((a: Post, b: Post) => {
    let ts1 = Date.parse(a.created_at);
    let ts2 = Date.parse(b.created_at);
    return ts2 - ts1;
  });
  tags.value = await TagApi.fetch_tags();
})
</script>

<template>
  <Header />
  <main>
    <h3 class='glow px-5 mt-3'>Посты</h3>
    <!-- <p><i class="nf nf-linux-archlinux"></i> ~/rust > <BlinkingCursor/></p> -->
    <hr>
    <h4 class='glow px-5'>Теги</h4>
    <TagList class='px-5 mt-3' :tags="tags" clickable />
    <hr>
    <PostList :posts="posts" />
  </main>
</template>

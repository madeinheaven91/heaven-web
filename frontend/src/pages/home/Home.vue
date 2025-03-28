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
  posts.value = await PostApi.fetch_posts();
  tags.value = await TagApi.fetch_tags();
})
</script>

<template>
  <Header/>
  <main>
    <h1 class='glow px-5'>Посты</h1>
    <!-- <p><i class="nf nf-linux-archlinux"></i> ~/rust > <BlinkingCursor/></p> -->
    <hr>
    <h4 class='glow px-5'>Теги</h4>
    <TagList :tags="tags" clickable/>
    <hr>
    <PostList :posts="posts"/>
  </main>
</template>

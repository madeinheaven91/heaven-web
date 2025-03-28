<script setup lang="ts">
import { type Post } from '@/entities/post';
import { onMounted, ref } from 'vue'
import TagList from "@/widgets/tag-list";

const props = defineProps<{ post: Post }>();
const strDate = ref("");
const displayTitle = ref("");
onMounted(() => {
  let ts = Date.parse(props.post.created_at);
  let date = new Date(ts);
  strDate.value = date.toLocaleDateString();
  displayTitle.value = (props.post.title.length > 16) ? props.post.title.slice(0, 16) + "..." : props.post.title;
})
</script>

<template>
    <div class="flex justify-between post">
      <div>
        <a :href="`/blog/post/${props.post.slug}`">{{ displayTitle  }}</a>
        <TagList :tags="props.post.tags"/>
        <p>{{ props.post.author.name }}</p>
      </div>
      <div>
        <p>{{ strDate }}</p>
      </div>
    </div>
</template>

<style lang="css" scoped>
a {
  text-decoration: none;
  color: var(--main);
  text-shadow: none;
  font-size: var(--f-xl);
  font-style: bold;
}
p {
  opacity: 0.8;
}
.post {
  /* @apply border-2; */
  padding-inline: 1rem;
  margin-bottom: 1rem;
}
</style>

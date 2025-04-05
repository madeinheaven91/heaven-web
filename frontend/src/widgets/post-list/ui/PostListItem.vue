<script setup lang="ts">
import { type Post, PostApi } from "@/entities/post";
import { type User } from "@/entities/user";
import { toLocaleDate } from "@/shared/lib/date";
import Button from "@/shared/ui/button";
import DeleteButton from "@/shared/ui/delete-button";
import TagList from "@/widgets/tag-list";
import { useAuthStore } from "@/shared/store.ts";
import { onMounted, ref, computed } from "vue";

const props = defineProps<{ post: Post }>();
const strDate = ref("");
const displayTitle = ref("");

const store = useAuthStore();
const user = computed(() => store.user);
const isAuthenticated = computed(() => store.isAuthenticated);

function onPublish(slug: string) {
  try {
    PostApi.publish(slug);
    alert("SUCCESS");
    window.location.replace(`/blog/post/${slug}`);
  } catch (error) {
    console.log(error);
  }
}

onMounted(() => {
  strDate.value = toLocaleDate(props.post.created_at);
  displayTitle.value =
    props.post.title.length > 16
      ? props.post.title.slice(0, 16) + "..."
      : props.post.title;
});
</script>

<template>
  <div class="flex justify-between post py-2">
    <div>
      <span class="flex items-center gap-5">
        <RouterLink :to="`/blog/post/${props.post.slug}`">{{ displayTitle }}</RouterLink>
        <TagList class="inline" :tags="props.post.tags" />
      </span>
      <p class="author-name">{{ props.post.author.name }}</p>
    </div>
    <div class="flex flex-col items-end justify-between py-2">
      <p class="no-margin">{{ strDate }}</p>
      <span class="flex gap-2" v-if="isAuthenticated">
        <Button v-if="user.id == props.post.author.id && !props.post.is_published"
          @click="onPublish(props.post.slug)">Опубликовать</Button>
        <Button v-if="user.id == props.post.author.id">
          <RouterLink class="edit" :to="`/blog/post/${props.post.slug}/edit`">Редактировать</RouterLink>
        </Button>
        <DeleteButton v-if="user.id == props.post.author.id || user.is_staff" :slug="props.post.slug" />
      </span>
    </div>
  </div>
</template>

<style lang="css" scoped>
.no-margin {
  margin: 0;
}

.red-btn {
  color: var(--red);
}

.red-btn:hover {
  border-bottom-color: var(--red);
}

.edit {
  text-decoration: none;
  color: var(--main);
  text-shadow: none;
  font-size: var(--f-m);
}

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

.author-name {
  margin-top: 0;
}
</style>

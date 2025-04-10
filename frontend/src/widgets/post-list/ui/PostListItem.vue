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
  let titleLength = 30
  displayTitle.value =
    props.post.title.length > titleLength
      ? props.post.title.slice(0, titleLength) + "..."
      : props.post.title;
});
</script>

<template>
  <!-- MOBILE -->
  <div class="flex flex-col justify-between post py-2 px-2 mb-3 border-2 lg:hidden">
    <div>
      <span class="flex flex-col lg:gap-5 lg:items-center">
        <RouterLink class="title text-xl lg:text-2xl" :to="`/blog/post/${props.post.slug}`">{{ displayTitle }}</RouterLink>
        <TagList class="mt-0" :tags="props.post.tags" />
      </span>
    </div>
    <div class="flex flex-col pt-4 pb-1">
      <div class="flex justify-between">
        <p class="author-name">{{ props.post.author.name }}</p>
        <p>{{ strDate }}</p>
      </div>
      <div class="flex gap-2 text-sm" v-if="isAuthenticated">
        <Button v-if="(user as User).id == props.post.author.id && !props.post.is_published"
          @click="onPublish(props.post.slug)">Опубликовать</Button>
        <Button v-if="(user as User).id == props.post.author.id">
          <RouterLink class="edit" :to="`/blog/post/${props.post.slug}/edit`">Редактировать</RouterLink>
        </Button>
        <DeleteButton v-if="(user as User).id == props.post.author.id || (user as User).is_staff" :slug="props.post.slug" />
      </div>
    </div>
  </div>
  <!-- PC -->
  <div class="justify-between post py-2 px-5 mb-3 border-2 text-2xl hidden lg:flex"> 
    <div>
      <span class="flex flex-row gap-5 items-center">
        <RouterLink class="title text-4xl" :to="`/blog/post/${props.post.slug}`">{{ displayTitle }}</RouterLink>
        <TagList class="mt-0" :tags="props.post.tags" />
      </span>
      <p>{{ props.post.author.name }}</p>
    </div>
    <div class="flex flex-col items-end justify-between py-2">
      <p>{{ strDate }}</p>
      <span class="flex flex-row gap-2 text-xl" v-if="isAuthenticated">
        <Button v-if="(user as User).id == props.post.author.id && !props.post.is_published"
          @click="onPublish(props.post.slug)">Опубликовать</Button>
        <Button v-if="(user as User).id == props.post.author.id">
          <RouterLink class="edit" :to="`/blog/post/${props.post.slug}/edit`">Редактировать</RouterLink>
        </Button>
        <DeleteButton v-if="(user as User).id == props.post.author.id || (user as User).is_staff" :slug="props.post.slug" />
      </span>
    </div>
  </div>
</template>

<style lang="css" scoped>
.edit {
  text-decoration: none;
  color: var(--main);
  text-shadow: none;
}

.title {
  text-decoration: none;
  color: var(--main);
  text-shadow: none;
  font-style: bold;
}

</style>

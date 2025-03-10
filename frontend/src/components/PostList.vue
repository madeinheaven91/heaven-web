<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import TagList from './TagList.vue'
import { type User } from '../shared/models.ts'
import { delete_post } from '../shared/utils.ts'
import { useStore } from 'vuex'
import DeleteButton from './DeleteButton.vue'
import axios from 'axios'
import { useRouter } from 'vue-router';

const router = useRouter();

const props = defineProps({
  posts: Array,
});

const store = useStore();
const user = computed(() => store.state.user);

const on_delete = async (slug) => {
  confirm('Вы уверены?');
  try {
    await delete_post(slug);
    alert("Удалено");
  }
  catch (error){
    console.log(error);
  }
};

const publish = async (slug) => {
  try {
    axios.patch(`http://localhost:8000/api/v1/blog/posts/${slug}`, {
      is_published: true
    }, {
      headers: {
        Authorization: `Bearer ${localStorage.getItem("access_token")}`,
      },
    });
    router.push(`/blog/post/${slug}`)
    alert("Опубликовано")
  } catch (error) {
    console.log(error)
  }

}
</script>

<template>
  <div v-if="posts && posts.length">
    <div v-for="post in posts" :key="post.slug">
      <div class="border px-3 pt-3 d-flex flex-row justify-content-between align-items-center w-auto">
        <div class="d-flex flex-column">
          <a :href="`/blog/post/${post.slug}`" class="text-decoration-none text-primary fs-3 text-wrap text-break">
              {{ post.title.length > 32 ? post.title.slice(0, 32) + '...' : post.title }}
          </a>
          <TagList :tags="post.tags" />
          <p class="text-secondary fs-5">{{ post.author.name }}</p>
        </div>
        <div v-if="user" class="d-flex gap-2">
          <div v-if="user.id == post.author.id" class="d-flex gap-2">
            <a v-if="!post.is_published" @click="publish(post.slug)" class="btn btn-success">Опубликовать</a>
            <a :href="`/blog/post/${post.slug}/edit`" class="btn btn-primary">Редактировать</a>
          </div>
          <DeleteButton v-if="user.id == post.author.id || user.is_staff" :slug="post.slug" />
        </div>
      </div>
    </div>
  </div>
  <p v-else class="fs-2 text-secondary">Нет постов...</p>
</template>

<script lang="ts" setup>
import { ref, onMounted, reactive, computed } from 'vue'
import { MdEditor } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'
import Header from '@/widgets/header'
import Button from '@/shared/ui/button'
import { type Post, PostApi } from '@/entities/post'
import { useAuthStore } from '@/shared/store.ts'
import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter();

const props = defineProps<{ slug: String | null }>();
const post = reactive<Post>({} as Post);
const store = useAuthStore();
// const user = computed(() => store.user);
const isAuthenticated = computed(() => store.isAuthenticated);

const title = ref("");
const body = ref("");
const BASE_URL = import.meta.env.VITE_API_URL;

const savePost = async () => {
  if (!title.value || !body.value) {
    alert("Заполните все поля");
    return;
  }
  if (props.slug) {
    try {
      await axios.patch(`${BASE_URL}/blog/posts/${props.slug}`, {
        title: title.value,
        body: body.value,
      }, {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("access_token")}`,
        },
      });
      alert("SUCCESS");
    } catch (error) {
      console.log(error);
    }
  } else {
    try {
      const response = await axios.post(`${BASE_URL}/blog/posts/new`, {
        title: title.value,
        body: body.value,
      }, {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("access_token")}`,
        },
      });
      const slug = response.data.slug;

      router.push(`/blog/post/${slug}`)
      alert("SUCCESS");
    } catch (error) {
      console.log(error);
    }
  }
}

onMounted(async () => {
  if (props.slug) {
    let p = await PostApi.fetch_post(props.slug as string);
    Object.assign(post, p);
    title.value = post.title;
    body.value = post.body;
  }
})
</script>

<template>
  <Header />
  <main>
    <div v-if="isAuthenticated">
      <input type="text" placeholder="Название" v-model="title" class="text-wrap text-break fs-1 w-full"></input>
      <p class="text-wrap text-break text-secondary fs-3">{{ store.user?.name }}</p>

      <hr />
      <MdEditor v-model="body" language="en-US" />
      <div class="mt-3 flex gap-2">
        <Button @click="savePost">Сохранить</Button>
        <Button @click="router.push('/blog')">Назад</Button>
      </div>
    </div>
    <div v-else>
      СЮДА ТЕБЕ НЕЛЬЗЯ
    </div>
  </main>
</template>

<style scoped>
input {
  font-size: var(--f-3xl);
}
</style>

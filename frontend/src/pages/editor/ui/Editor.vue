<script lang="ts" setup>
import { ref, onMounted, reactive, computed } from 'vue'
import { MdEditor } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'
import Header from '@/widgets/header'
import { type Tag } from '@/entities/tag'
import { type Post, PostApi } from '@/entities/post'
import { useAuthStore } from '@/shared/store.ts'
import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter();

const props = defineProps<{ slug: String | null }>();
const post = reactive<Post>({} as Post);
const store = useAuthStore();
const user = computed(() => store.user);
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
    try{
      const response = await axios.patch(`${BASE_URL}/blog/posts/${props.slug}`, {
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
  }else{
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

      for (const tag of selectedTags.value) {
        await axios.post(`${BASE_URL}/blog/posts/${slug}/assign/${tag.slug}`, {}, {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("access_token")}`,
          },
        });
      }
      router.push(`/blog/post/${slug}`)
      alert("SUCCESS");
    } catch (error) {
      console.log(error);
    }
  }
}

const allTags = ref<Tag[]>([]);
const selectedTags = ref<Tag[]>([]);
const selectedTag = ref<Tag | null>(null);

const addTag = (tag: Tag) => {
  if (!selectedTags.value.some(t => t.slug === tag.slug)) {
    selectedTags.value.push(tag);
  }
}

const removeTag = (tag: Tag) => {
  selectedTags.value = selectedTags.value.filter(t => t.slug != tag.slug)
}

onMounted(async () => {
  try {
    const response = await axios.get(`${BASE_URL}/blog/tags/fetch`);
    allTags.value = response.data;
  } catch (error) {
    console.log(error);
  }

  if (props.slug) {
    let p = await PostApi.fetch_post(props.slug as string);
    Object.assign(post, p);
    title.value = post.title;
    body.value = post.body;
  }
})
</script>

<template>
  <Header/>
  <div v-if="isAuthenticated">
    <input type="text" v-model="title" class="text-wrap text-break fs-1"></input>
    <p class="text-wrap text-break text-secondary fs-3">{{ store.user?.name }}</p>

    <div class="dropdown">
      <button class="btn btn-secondary dropdown-toggle" type="button" id="dropdownMenuButton1" data-bs-toggle="dropdown"
        aria-expanded="false">
        Добавить теги
      </button>
      <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
        <li v-for="tag in allTags" :key="tag.slug">
          <a @click="addTag(tag)" class="badge mr-1" :style="{
            textDecoration: 'none',
            backgroundColor: '#' + tag.background_color,
            color: '#' + tag.foreground_color
          }">
            {{ tag.name }}
          </a>
        </li>
      </ul>
    </div>

    <div class="d-flex gap-2 mt-3">
      <span v-for="tag in selectedTags" :key="tag.slug">
        <a @click="removeTag(tag)" class="badge mr-1" :style="{
          textDecoration: 'none',
          backgroundColor: '#' + tag.background_color,
          color: '#' + tag.foreground_color
        }">
          {{ tag.name }}
        </a>
      </span>
    </div>

    <hr />
    <MdEditor v-model="body" language="en-US" />
    <div class="mt-3 d-flex gap-2">
      <a @click="savePost" class="btn btn-success">Сохранить</a>
      <a onclick="history.back()" class="btn btn-primary">Назад</a>
    </div>
  </div>
  <div v-else>
    СЮДА ТЕБЕ НЕЛЬЗЯ
  </div>
</template>

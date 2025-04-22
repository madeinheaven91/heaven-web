<script setup lang="ts">
import Button from '@/shared/ui/button'
import DeleteButton from "@/shared/ui/delete-button";
import { type Post, PostApi } from '@/entities/post'
import { type Tag } from '@/entities/tag'
import { type User } from "@/entities/user";
import Header from '@/widgets/header'
import TagList from '@/widgets/tag-list'
import { DateLib } from '@/shared/lib'
import { MdPreview } from 'md-editor-v3'
import { onMounted, ref, computed } from 'vue'
import { useAuthStore } from "@/shared/store.ts";
import { useRouter } from 'vue-router'

const router = useRouter();
const store = useAuthStore();
const user = computed(() => store.user);

const props = defineProps({
    slug: String
});
const tags = ref<Tag[]>([]);
const post = ref<Post | null>(null);
onMounted(async () => {
    if (!props.slug) return;
    let p = await PostApi.fetch_post(props.slug);
    // Object.assign(post, p);
    post.value = p;
    tags.value = p.tags;
})
</script>

<template>
    <div class="flex flex-col min-h-screen">
        <Header />
        <main class="px-5 lg:px-50">
            <template v-if="post">
                <h1 class="break-normal mt-5 text-3xl font-bold lg:text-5xl">{{ post.title }}</h1>
                <TagList class="my-3" :tags="tags" />
                <hr>
                <h4 class="text-2xl lg:text-3xl">{{ post.author.name }}</h4>
                <p class="my-0 text-xs italic lg:text-base">Создано: {{ DateLib.toLocale(post.created_at) }}</p>
                <p class="my-0 text-xs italic lg:text-base" v-if="post.updated_at">Обновлено: {{
                    DateLib.toLocale(post.updated_at) }}</p>
                <hr>
                <MdPreview v-model="post.body" language='en-US' preview-theme="heaven" code-theme="hacker" class="lg:text-lg" />
                <hr class="mt-5">
                <div class="my-5 flex gap-3 text-xl lg:text-2xl">
                    <Button @click="router.push('/blog')">Назад</Button>
                    <template v-if="user">
                        <Button v-if="(user as User).id == post.author.id">
                            <RouterLink class="edit" :to="`/blog/post/${props.slug}/edit`">Редактировать</RouterLink>
                        </Button>
                        <DeleteButton v-if="(user as User).id == post.author.id || (user as User).is_staff"
                            :slug="slug" />
                    </template>
                </div>
            </template>
            <template v-else>
                <h1>Пост не найден</h1>
            </template>
        </main>
        <footer class="bg-(--bg-darker)">
            <div class="px-5 py-5 lg:px-50 flex gap-3 flex-col lg:flex-row items-center justify-between">
                <p class="block lg:hidden">Понравилась статья? <a href="https://t.me/jazz_coding" target="_blank">Оставь
                        отзыв</a></p>
                <p class="block lg:hidden">made in heaven</p>
                <p class="hidden lg:block">Понравилась статья? <a href="https://t.me/jazz_coding" target="_blank">Оставь
                        отзыв</a></p>
                <p class="hidden lg:block">made in heaven</p>
                <div class="flex flex-row gap-3">
                    <a href="https://t.me/madeinheaven91" target="_blank"><img width="48px"
                            src="/public/icons/telegram.svg" class="filter-main" /></a>
                    <a href="https://github.com/madeinheaven91" target="_blank"><img width="48px"
                            src="/public/icons/github.svg" class="filter-main" /></a>
                </div>
            </div>
        </footer>
    </div>
</template>

<style lang="css" scoped>
main {
    flex: 1;
}

.md-editor-previewOnly {
    background-color: transparent;
}

.edit {
    text-decoration: none;
    color: var(--main);
    text-shadow: none;
}
</style>

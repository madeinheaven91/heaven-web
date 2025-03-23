import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import Post from "@/pages/Post.vue";
import Editor from "@/pages/Editor.vue";
import Drafts from "@/pages/Drafts.vue";
import NotFound from "@/pages/NotFound.vue";

const routes = [
  { path: "/", redirect: "/blog", },
  { path: "/blog", component: Home, },
  { path: "/blog/new", component: Editor, },
  { path: "/blog/drafts", component: Drafts, },
  {
    path: "/blog/post/:slug",
    component: Post,
    props: true
  },
  {
    path: "/blog/post/:slug/edit",
    component: Editor,
    props: true
  },
  { path: '/:pathMatch(.*)*', component: NotFound, }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

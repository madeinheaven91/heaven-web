import { createRouter, createWebHistory } from "vue-router";
import Home from '@/pages/home'
import PostDetail from "@/pages/post-detail"
import Editor from "@/pages/editor"
import Drafts from "@/pages/drafts"

const routes = [
  { path: "/", redirect: "/blog", },
  { path: "/blog", component: Home, },
  { path: "/blog/new", component: Editor, },
  { path: "/blog/drafts", component: Drafts, },
  {
    path: "/blog/post/:slug",
    component: PostDetail,
    props: true
  },
  {
    path: "/blog/post/:slug/edit",
    component: Editor,
    props: true
  },
  // { path: '/:pathMatch(.*)*', component: NotFound, }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

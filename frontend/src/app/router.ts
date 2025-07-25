import { createRouter, createWebHistory } from "vue-router";
import BlogHome from "@/pages/blog-home";
import PostDetail from "@/pages/post-detail";
import Editor from "@/pages/editor";
import Drafts from "@/pages/drafts";

const routes = [
  {
    path: "/",
    component: BlogHome,
  },
  { path: "/new", component: Editor },
  { path: "/drafts", component: Drafts },
  {
    path: "/post/:slug",
    component: PostDetail,
    props: true,
  },
  {
    path: "/post/:slug/edit",
    component: Editor,
    props: true,
  },
  // { path: '/:pathMatch(.*)*', component: NotFound, }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

import { createRouter, createWebHistory } from "vue-router";
import HomeView from "./views/HomeView.vue";
import PostView from "./views/PostView.vue";
import EditorView from "./views/EditorView.vue";
import DraftsView from "./views/DraftsView.vue";
import NotFoundView from "./views/NotFoundView.vue";

const routes = [
  { path: "/", redirect: "/blog", },
  { path: "/blog", component: HomeView, },
  { path: "/blog/new", component: EditorView, },
  { path: "/blog/drafts", component: DraftsView, },
  {
    path: "/blog/post/:slug",
    component: PostView,
    props: true
  },
  {
    path: "/blog/post/:slug/edit",
    component: EditorView,
    props: true
  },
  { path: '/:pathMatch(.*)*', component: NotFoundView, }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

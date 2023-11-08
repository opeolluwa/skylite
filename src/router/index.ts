import IndexPage from "../views/index.vue";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "IndexPage",
    component: IndexPage,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

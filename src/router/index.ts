import IndexPage from "../views/index.vue";
import FileUploadPage from "../views/upload.vue";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "IndexPage",
    component: IndexPage,
  },
  {
    path: "/upload",
    name: "FileUploadPAge",
    component:FileUploadPage,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

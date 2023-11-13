import { createApp } from "vue";
// import "./styles.css";
// import "./index.css";

import PrimeVue from "primevue/config";

import App from "./Upload.vue";

const app = createApp(App);
app.use(PrimeVue);
app.mount("#app");

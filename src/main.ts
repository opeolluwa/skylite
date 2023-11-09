import { createApp } from "vue";
// import "./styles.css";
import "./index.css";

import PrimeVue from "primevue/config";

import App from "./App.vue";
import router from "./router";

import { emit, listen } from "@tauri-apps/api/event";


(async () => {
  await listen("device-connected", (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event.payload)
  });
})();

const app = createApp(App);
app.use(PrimeVue);
app.use(router);
app.mount("#app");

const { createApp, ref } = Vue;
import Dropzone from "../components/dropzone.js";

createApp({
  setup() {
    const message = ref('Hello vue!')
    const deviceName = ref("Sillicone")
    const navTabs = [
      {name:'home', path:'home'},
      {name:'audio', path:'video'},
      {name:'video', path:'video'},
      {name:'document', path:'document'}
    ]
    return {
      message,
      navTabs,
      Dropzone,
      deviceName
    }
  }
}).mount('#app')
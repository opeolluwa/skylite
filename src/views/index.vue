<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { onMounted, ref } from 'vue';
import { CommandData } from '../CommandData';
import { message } from '@tauri-apps/api/dialog';
import vueQr from 'vue-qr/src/packages/vue-qr.vue'
import router from '../router';

const serverAddress = ref("");
const isConnectedToWifi = ref(false);

onMounted(async ()=>{
await invoke('server').then((response) =>{
  const ipcResponse = response as unknown as CommandData<string>;
  serverAddress.value = ipcResponse.data as unknown as string
})

//TODO: fetch data from backend 
await invoke('is_connected_to_wifi').then((response) =>{
  const ipcResponse = response as unknown as CommandData<boolean>;
  isConnectedToWifi.value = ipcResponse.data as unknown as boolean
})

//TODO: reroute on successfully connection
function qrScanCallback(){
  // route to /upload
  router.push({path:"/upload"})
}

// show a message dialog
await message(`visit ${serverAddress.value} on the connected device`, { title: 'skylite', type: 'info' });
})
</script>

<template>
  <div class="dark:text-gray-400" v-if="!isConnectedToWifi">
    <p class="capitalize left text-purple mt-10 text-[#FF3D00]">Waiting for connection</p>
    <p class="mt-4 py-4 w-3/4 dark:text-gray-400 leading-1">
      You should see the file transfer begin as soon as someone start transferring to your computer from their Wifi
      enabled device
    </p>

    <div class="flex flex-col items-center justify-center mt-[100px]">
      <span class="loader"></span>
    </div>






  </div>
  <div v-else class="flex flex-col items-center justify-center h-[100%]">
    <div class="flex flex-col items-center justify-center">
      <div>
        <vue-qr bgSrc='/satellite.png' dot-scale="1" class="border-r-2 mb-3" :text="serverAddress" :size="200"></vue-qr>
      </div>
      <h2 class="dark:text-gray-400">scan for visit <span class="text-[#FF3D00]">{{ serverAddress }}</span>
      </h2>
    </div>
  </div>
</template>


<style scoped>
.loader {
  box-sizing: border-box;
  display: inline-block;
  width: 40px;
  height: 80px;
  border-top: 5px solid #fff;
  border-bottom: 5px solid #fff;
  position: relative;
  background: linear-gradient(#FF3D00 30px, transparent 0) no-repeat;
  background-size: 2px 40px;
  background-position: 50% 0px;
  animation: spinx 5s linear infinite;
}

.loader:before,
.loader:after {
  content: "";
  width: 30px;
  left: 50%;
  height: 35px;
  position: absolute;
  top: 0;
  transform: translatex(-50%);
  background: rgba(255, 255, 255, 0.4);
  border-radius: 0 0 20px 20px;
  background-size: 100% auto;
  background-repeat: no-repeat;
  background-position: 0 0px;
  animation: lqt 5s linear infinite;
}

.loader:after {
  top: auto;
  bottom: 0;
  border-radius: 20px 20px 0 0;
  animation: lqb 5s linear infinite;
}

@keyframes lqt {

  0%,
  100% {
    background-image: linear-gradient(#FF3D00 40px, transparent 0);
    background-position: 0% 0px;
  }

  50% {
    background-image: linear-gradient(#FF3D00 40px, transparent 0);
    background-position: 0% 40px;
  }

  50.1% {
    background-image: linear-gradient(#FF3D00 40px, transparent 0);
    background-position: 0% -40px;
  }
}

@keyframes lqb {
  0% {
    background-image: linear-gradient(#FF3D00 40px, transparent 0);
    background-position: 0 40px;
  }

  100% {
    background-image: linear-gradient(#FF3D00 40px, transparent 0);
    background-position: 0 -40px;
  }
}

@keyframes spinx {

  0%,
  49% {
    transform: rotate(0deg);
    background-position: 50% 36px;
  }

  51%,
  98% {
    transform: rotate(180deg);
    background-position: 50% 4px;
  }

  100% {
    transform: rotate(360deg);
    background-position: 50% 36px;
  }
}
</style>
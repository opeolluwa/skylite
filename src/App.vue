<script setup lang="ts">
import { exit,relaunch } from '@tauri-apps/api/process';
import { ask} from '@tauri-apps/api/dialog';

async function close(){
  const yes= await ask('Current file transfer would be lost. Do you still want to proceed?', { title: 'Close', type: 'warning' });
  if(yes){
 await exit(1)

  }
}

async function refresh(){
await relaunch()
}
</script>


<template>
  <Layout class="px-[50px] py-[10px] dark:bg-[#2f2f2f] h-screen" id="main" style="display: block; padding: 20px 50px;">
    <RouterView></RouterView>
  </Layout>
  <footer class="bg-[#ddd] dark:bg-[#222] text-gray-600">
    <button @click="refresh" class="dark:bg-[#222]">Refresh</button>
    <button @click="close" class="dark:bg-[#222] hover:bg-transparent border-2 border-[#FF3D00]">Close</button>
  </footer>
</template>




<style scoped>
#main {
  padding: 20px 50px !important;
}
</style>
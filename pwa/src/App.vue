<script setup lang="ts">
import Button from 'primevue/button';
import TabMenu from 'primevue/tabmenu';
import 'primeicons/primeicons.css'
import {ref} from "vue";
import Layout from "./components/Layout.vue";
import {ShareIcon} from "@heroicons/vue/24/outline";
import {MagnifyingGlassIcon as SearchIcon} from "@heroicons/vue/24/outline";
import {ArrowUpTrayIcon as UploadIcon} from "@heroicons/vue/24/outline";
//TODO: fetch form the backend
const connectedDeviceName = ref("Sillicone");
</script>

<template>
<Layout>
    <header class="h-[100px]">
 <div class="flex justify-between items-center">
   <h1 class="text-2xl font-bold leading">
     {{connectedDeviceName}}
   </h1>
<div class="flex gap-[6px]">
  <SearchIcon class="w-6 h-6"/>
<!--  <UploadIcon class="w-6 h-6 " />-->
<!--  <ShareIcon class="w-6 h-6 "/>-->
</div>

 </div>
      <div class=" mt-[45px] flex flex-wrap justify-between -mb-px text-sm font-medium text-center text-gray-500 dark:text-gray-400">
        <span class="inline-flex gap-2 items-center justify-center border-b-[#FF3D00] text-[#FF3D00]  border-b-2">
          Images
        </span>
        <span class="inline-flex gap-2 items-center justify-center">
          Audio
        </span>
        <span class="inline-flex gap-2 items-center justify-center">
          Videos
        </span>
        <span class="inline-flex gap-2 items-center justify-center">
          Documents
        </span>
      </div>
    </header>

</Layout>
</template>

<style scoped>

</style>

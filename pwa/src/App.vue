<script setup lang="ts">
import  {ref} from "vue";

import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';

import {MagnifyingGlassIcon} from "@heroicons/vue/24/outline"
const deviceName = ref("Sillicone")

const tabs = [
  {
    title:"Video",
    content:"  aliqua. Ut enim ad minExcepteur sint occaecat cupidatat non proident, sun"
  },
  {
    title:"Audio",
    content:"  aliqua. Ut enim ad minExcepteur sint occaecat cupidatat non proident, sun"
  },
  {
    title:"Music",
    content:"  aliqua. Ut enim ad minExcepteur sint occaecat cupidatat non proident, sun"
  },
  {
    title:"Document",
    content:"  aliqua. Ut enim ad minExcepteur sint occaecat cupidatat non proident, sun"
  }
]
</script>

<template>
  <header class="flex items-center justify-between px-[20px] py-[30px]">
    <h1 class="">
      {{deviceName}}
    </h1>
    <MagnifyingGlassIcon class="w-6 h-6"/>
  </header>

  <div class="card">
    <TabView scrollable>
      <TabPanel :header="tab.title" v-for="tab in tabs">
        <p class="m-0">
        {{tab.content}}
        </p>
      </TabPanel>



    </TabView>
  </div>

</template>




<style scoped>
header{
  background: #F6F8FD;

}
.p-tabview-nav {
  color:red!important
}
h1{
  line-height: 24px;
  font-weight:600;
  font-size:18px;
}
</style>
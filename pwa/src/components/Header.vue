<template>
  <header class="flex items-center justify-between h-[100px] px-[50px] bg-[#ddd]">
<h1 class="">
  {{deviceName}}
</h1>
    <MagnifyingGlassIcon class="w-[6px] hidden"/>
  </header>
</template>

<script setup lang="ts">
import  {ref} from "vue";
import {MagnifyingGlassIcon} from "@heroicons/vue/24/outline"
const deviceName = ref("Sillicone")
</script>


<style scoped>
header{
  background: #F6F8FD;

}
h1{
  line-height: 24px;
  font-weight:600;
  font-size:18px;
}
</style>
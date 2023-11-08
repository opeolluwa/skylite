<template>
  <Layout class="px-[50px] py-[10px] dark:bg-[#2f2f2f] h-screen" id="main" style="display: block; padding: 20px 50px;">
    <RouterView></RouterView>
  </Layout>
  <footer class="bg-[#ddd] dark:bg-[#222] text-gray-600">
    <button>Next</button>
    <button>Cancel</button>
  </footer>
</template>




<style scoped>
#main {
  padding: 20px 50px !important;
}
</style>
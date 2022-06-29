<template>
  <div id="app">
    <CommonMeun class="menu"/>
    <router-view class="pages"></router-view>
  </div>
</template>

<script>
import CommonMeun from './components/CommonMeun.vue'

export default {
  name: 'App',
  components: {
    CommonMeun
}
}
</script>

<style>
#app{
  display: flex;
  flex-direction: columns;
}
.menu{
  width: 26vw;
  height: 100vh;
  max-width: 300px;
}
.pages{
  width: 74vw;
  height: 100vh;
}
</style>

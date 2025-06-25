<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue';
</script>

<template>



  <div class="min-h-screen flex flex-col">
    <!-- <NavBar /> 是顶级组件，它会一直显示在页面上，无论用户访问哪个路径。-->
    <NavBar />
    <!-- <router-view /> 是vue-router 提供的占位组件，表示“当前路由匹配到的页面组件”应该渲染在这里。-->
    <router-view />
  </div>



</template>

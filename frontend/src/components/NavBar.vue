<template>
    <nav class="bg-white shadow px-6 py-3 flex justify-center items-center">
        <router-link to="/" class="text-xl font-bold text-blue-600">我的博客</router-link>

        <div class="flex items-center space-x-4">
            <router-link v-if="isLoggedin" to="/admin" class="text-sm text-gray-600 hover:text-blue-600">后台管理</router-link>

            <button v-if="isLoggedin" @click="handleLogout" class="text-sm text-gray-600 hover:text-red-600">退出</button>

            <router-link v-else to="/login" class="text-sm text-gray-600 hover:text-blue-600">登录</router-link>
        </div>
    </nav>
</template>


<script setup lang="ts">
import { useUserStore } from '@/stores/user';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

const userstore = useUserStore();
const router = useRouter();

// !!user.token 是布尔值，专门用来表示“是否登录”这个状态
const isLoggedin = computed(() => !!userstore.token)
const handleLogout = () => {
    userstore.logout();
    router.push('/');
}

</script>
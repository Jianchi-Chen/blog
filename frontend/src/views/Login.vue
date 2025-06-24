<template>
    <div class="flex items-center justify-center min-h-screen bg-gray-900">
        <div class="w-full max-w-md p-8 bg-emerald-700 rounded shadow">
            <h1 class="mb-6 text-2xl font-bold text-center">登录后台</h1>

            <div v-if="errorMessage" class="mb-4 text-red-500 text-sm">{{ errorMessage }}</div>


            <form @submit.prevent="handleLogin" class="space-y-4">
                <div>
                    <label class="block mb-1 text-sm font-medium">用户名</label>
                    <input type="text" v-model="username"
                        class="w-full px-4 py-2 border rounded outline-none focus:ring  focus:border-blue-400"
                        required />
                </div>

                <div>
                    <label>密码</label>
                    <input type="password" v-model="password"
                        class="w-full px-4 py-2 border rounded outline-none focus:ring  focus:border-blue-400"
                        required />
                </div>

                <button type="submit" class=" w-full py-2 text-white bg-blue-600 rounded hover:bg-blue-700"
                    :disabled="loading"> {{ loading ? '登陆中' : '登录' }}</button>
            </form>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user';
import axios from 'axios';
import { ref } from 'vue';
import { useRouter } from 'vue-router';


// 状态
const username = ref('');
const password = ref('');
const loading = ref(false);
const errorMessage = ref('');

//引入Pinia Store和Vue Router
const userStore = useUserStore();
const router = useRouter();

//登录函数
const handleLogin = async () => {
    errorMessage.value = '';
    loading.value = true;

    try {

        // 暂时使用模拟请求（你后续用真实 API 替换）
        const response = await axios.post('/login', {
            username: username.value,
            password: password.value,
        })
        console.log('[请求拦截]', response.data.url);


        // 模拟返回：{ token: "xxx", username: "admin" }
        // 解构+重命名：从 response.data 中取出 username 字段，赋值给变量 name
        const { token, username: name } = response.data;

        userStore.login(token, name);
        router.push('/');
    } catch (err) {
        errorMessage.value = '用户名或者密码错误';
    } finally {
        loading.value = false;
    }
}

</script>

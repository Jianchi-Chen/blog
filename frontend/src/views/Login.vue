<template>
    <!-- naive-ui实现 -->
    <n-layout style="height: 100vh;">
        <n-layout-content position="absolute" class="w-full h-full flex items-center justify-center bg-gray-50">
            <div class="bg-white shadow-md rounded-lg p-8 w-full max-w-md">
                <h1 class="text-2xl font-bold mb-6 text-center"> 登录后台</h1>
                <n-form :model="form" :rules="rules" ref="formRef" label-placement="top">
                    <n-form-item label="用户名" path="username">
                        <n-input v-model:value="form.username" placeholder="请输入用户名" />
                    </n-form-item>
                    <n-form-item label="密码" path="password">
                        <n-input v-model:value="form.password" placeholder="请输入密码" />
                    </n-form-item>


                    <n-form-item>
                        <n-button type="primary" @click="handleLogin" :loading="loading">登录</n-button>
                    </n-form-item>
                </n-form>
            </div>
        </n-layout-content>
    </n-layout>
</template>

<script setup lang="ts">
import { NForm, NFormItem, NInput, NButton, useMessage, NLayout, NLayoutContent } from 'naive-ui';
import { useUserStore } from '@/stores/user';
import axios from 'axios';
import { ref } from 'vue';
import { useRouter } from 'vue-router';


// 状态
const loading = ref(false);

//引入Pinia Store和Vue Router
const userStore = useUserStore();
const router = useRouter();

// Nform表单
const form = ref({
    username: '',
    password: '',
})
const rules = {
    username: {
        required: true,
        message: '请输入用户名',
        trigger: ['input', 'blur'],
    },
    password: {
        required: true,
        message: '请输入密码',
        trigger: ['input', 'blur'],
    }
}
const message = useMessage();
const formRef = ref();


//登录函数
const handleLogin = async () => {
    loading.value = true;

    try {
        // .validate()验证表单
        await formRef.value?.validate()

        // 暂时使用模拟请求（你后续用真实 API 替换）
        const response = await axios.post('/login', {
            ...form.value,
        })
        console.log('[请求拦截]', response.data.url);

        // 模拟返回：{ token: "xxx", username: "admin" }
        // 解构+重命名：从 response.data 中取出 username 字段，赋值给变量 name
        const { token, username: name } = response.data;

        userStore.login(token, name);
        message.success('登录成功')
        router.push('/');
    } catch (err) {
        message.error('登录失败')
    } finally {
        loading.value = false;
    }
}

</script>

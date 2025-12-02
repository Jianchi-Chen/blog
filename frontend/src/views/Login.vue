<template>
    <div class="h-screen w-[90vh] grid place-items-center items-start">
        <n-flex class="w-[50vh]">
            <n-card>
                <n-tabs default-value="signin" size="large" animated>
                    <n-tab-pane name="signin" tab="登录">
                        <n-form
                            :model="signinForm"
                            :rules="signinRules"
                            ref="signinRef"
                        >
                            <n-form-item-row label="用户名" path="username">
                                <n-input
                                    v-model:value="signinForm.username"
                                    placeholder="请输入用户名"
                                />
                            </n-form-item-row>
                            <n-form-item-row label="密码" path="password">
                                <n-input
                                    v-model:value="signinForm.password"
                                    placeholder="请输入密码"
                                />
                            </n-form-item-row>
                        </n-form>
                        <n-button
                            type="primary"
                            block
                            secondary
                            strong
                            @click="handleLogin"
                        >
                            登录
                        </n-button>
                    </n-tab-pane>

                    <n-tab-pane name="signup" tab="注册">
                        <n-form
                            :model="registerForm"
                            :rules="registerRules"
                            ref="registerRef"
                        >
                            <n-form-item-row label="用户名" path="username">
                                <n-input
                                    v-model:value="registerForm.username"
                                    placeholder="请输入用户名"
                                />
                            </n-form-item-row>
                            <n-form-item-row label="密码" path="password">
                                <n-input
                                    v-model:value="registerForm.password"
                                    placeholder="请输入用户名"
                                />
                            </n-form-item-row>
                            <n-form-item-row label="重复密码" path="repassword">
                                <n-input
                                    v-model:value="registerForm.repassword"
                                    placeholder="请输入用户名"
                                />
                            </n-form-item-row>
                        </n-form>
                        <n-button
                            type="primary"
                            block
                            secondary
                            strong
                            @click="handleRegister"
                        >
                            注册
                        </n-button>
                    </n-tab-pane>
                </n-tabs>
            </n-card>
        </n-flex>
    </div>
</template>

<script setup lang="ts">
import {
    NForm,
    NFormItem,
    NInput,
    NButton,
    NFormItemRow,
    useMessage,
    NLayout,
    NLayoutContent,
    NFlex,
    NCard,
    NTabs,
    NTabPane,
} from "naive-ui";
import { useUserStore } from "@/stores/user";
import axios from "axios";
import { onMounted, ref, watchEffect } from "vue";
import { useRouter } from "vue-router";
import { loginAccount, registerAccount } from "@/api/account";

// 状态
const loading = ref(false);

//引入Pinia Store和Vue Router
const userStore = useUserStore();
const router = useRouter();

// const count = ref(1);

// 登录表单与验证逻辑
const signinForm = ref({
    username: "",
    password: "",
});
const signinRules = {
    username: {
        required: true,
        message: "请输入用户名",
        trigger: ["input", "blur"],
    },
    password: {
        required: true,
        message: "请输入密码",
        trigger: ["input", "blur"],
    },
};

// 注册表单与验证逻辑
const registerForm = ref({
    username: "",
    password: "",
    repassword: "",
});
const registerRules = {
    username: [
        {
            required: true,
            message: "请输入用户名",
            trigger: ["input", "blur"],
        },
        {
            min: 2,
            max: 20,
            message: "长度 2-20",
            trigger: ["blur", "input"],
        },
    ],
    password: [
        {
            required: true,
            message: "请输入密码",
            trigger: ["input", "blur"],
        },
        {
            min: 6,
            max: 20,
            message: "长度 6-20",
            trigger: ["blur", "input"],
        },
    ],
    repassword: [
        {
            required: true,
            message: "请确认密码",
            trigger: ["input", "blur"],
        },
        {
            // 验证密码逻辑
            validator: (rule: any, value: string) => {
                return registerForm.value.password === value
                    ? true
                    : new Error("两次密码输入不一致");
            },
            message: "两次密码输入不一致",
            trigger: "blur",
        },
    ],
};

const message = useMessage();
const signinRef = ref();
const registerRef = ref();

//登录函数
const handleLogin = async () => {
    loading.value = true;

    try {
        // .validate()验证表单
        await signinRef.value?.validate();

        // 暂时使用模拟请求（你后续用真实 API 替换）
        const response = await loginAccount(signinForm.value);

        if (response.data.message == "failed") {
            throw new Error("未注册用户");
        }

        console.log("[请求拦截]", response.data.url);

        // console.log(response.data); //tokenz

        // 模拟返回：{ token: "xxx", username: "admin" }
        // 解构+重命名：从 response.data 中取出 username 字段，赋值给变量 name
        const token = response.data.token;
        const name = { ...response.data.user };
        const identity = response.data.identity;

        userStore.login(token, response.data.user);
        console.log(userStore.identity);

        message.success("登录成功");
        router.push("/");
    } catch (err) {
        // 类型守卫
        if (err instanceof Error) {
            message.error(`${err}`);
        }
    } finally {
        loading.value = false;
    }
};

//注册用户
const handleRegister = async () => {
    loading.value = true;
    try {
        // .validate()验证表单
        await registerRef.value?.validate();
        const res = await registerAccount(registerForm.value);
        if (res.data.message == "failed") {
            throw new Error("注册失败");
        }
        message.success("注册成功,即将登录... go to Home");
        // console.log('[请求拦截]', res.data.url);
        const token = res.data.token;
        const name = { ...res.data.user };

        userStore.login(token, res.data.user);
        router.push("/");
    } catch (err) {
        console.error(err);
        message.error("注册失败");
    } finally {
        loading.value = false;
    }
};
</script>

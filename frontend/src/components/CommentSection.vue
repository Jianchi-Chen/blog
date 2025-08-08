<template>
    <n-layout>
        <n-h2>评论该文章</n-h2>
        <n-alert title="提示" v-if="!userhasLogin">登录后才能发表评论</n-alert>
        <n-flex v-else>
            <n-form :model="formData" :rules="formRules" ref="formRef">
                <n-form-item path="newComment">
                    <n-input type="textarea" v-model:value="formData.newComment" round placeholder="写下你的评论" bordered />
                    <n-button :loading="loading" @click="submitComment">{{ commentCoolDown > 0 ? `${commentCoolDown}秒冷却`
                        :
                        '发表评论' }}</n-button>
                </n-form-item>
            </n-form>
        </n-flex>

        <n-h2>评论区</n-h2>
        <n-card v-for="comment in comments" :key="comment.id">
            <n-p>{{ comment.content }}</n-p>
            <n-text type="success">来自: {{ comment.user }} | {{ comment.created_at }}</n-text>
        </n-card>
    </n-layout>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user';
import { NInput, NLayout, NFlex, NFormItem, NForm, NButton, NCard, useMessage, type FormInst, NAlert, NH2, NText, NP } from 'naive-ui';
import { onMounted, ref, type Ref } from 'vue';
import { fetchComments, postComment } from '@/api/comment';
import { useRoute, useRouter } from 'vue-router';

const userStore = useUserStore()
const userhasLogin = userStore.token ? true : false
const loading = ref(false)
const newComment = ref()
const route = useRoute()
const articleId = route.params.id as string
const message = useMessage()
const comments = ref<any[]>([])
const formRef = ref<FormInst | null>()
const commentCoolDown = ref(0)
const timer = ref(0)

const formData = ref({
    newComment: ''
})
const formRules = {
    newComment: {
        required: true,
        min: 2,
        message: '最短评论长度为 2'
    }
}

const submitComment = async () => {
    loading.value = true
    try {
        await formRef.value?.validate()
        await postComment(articleId, formData.value.newComment, userStore.username)
        message.success("评论成功")
        await loadComments()
    } catch (e) {
        if (e instanceof Error) {
            message.error(`${e}`)
        }
    } finally {
        // 开始倒计时
        commentCoolDown.value = 3
        timer.value = setInterval(() => {
            commentCoolDown.value--
            if (commentCoolDown.value <= 0) {
                clearInterval(timer.value as number)
            }
        }, 1000);

        formData.value.newComment = ""
        setTimeout(() => {
            loading.value = false
        }, 3000);
    }
}

const loadComments = async () => {
    const res = await fetchComments(articleId)
    comments.value = res.data
}

onMounted(() => {
    loadComments()
})

</script>
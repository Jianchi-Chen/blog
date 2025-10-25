<template>
    <n-layout class="w-[90%]">
        <n-h2>评论该文章</n-h2>
        <n-alert title="提示" v-if="!userhasLogin">登录后才能发表评论</n-alert>
        <n-flex v-else>
            <n-form :model="formData" :rules="formRules" ref="formRef" class="w-[90%]">
                <n-form-item path="newComment">
                    <n-input type="textarea" v-model:value="formData.newComment" round placeholder="写下你的评论" bordered />
                    <n-button :loading="loading" @click="submitComment">{{ commentCoolDown > 0 ? `${commentCoolDown}秒冷却`
                        :
                        '发表评论' }}</n-button>
                </n-form-item>
            </n-form>
        </n-flex>


        <n-h2>评论区</n-h2>
        <n-card v-for="comment in comments" :key="comment.comment_id">
            <n-p>{{ comment.content }}</n-p>
            <n-flex inline :wrap="false" justify="start" align="center">
                <n-text type="success">来自: {{ comment.user }} | {{ comment.created_at }}</n-text>
                <n-button v-if="userStore.identity == 'admin'" @click="handlerDeleteComment(comment.comment_id)">
                    删除</n-button>
            </n-flex>
        </n-card>


    </n-layout>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user';
import { NInput, NLayout, useDialog, NFlex, NFormItem, NForm, NButton, NCard, useMessage, type FormInst, NAlert, NH2, NText, NP } from 'naive-ui';
import { onMounted, ref, type Ref } from 'vue';
import { DeleteComment, fetchComments, postComment } from '@/api/comment';
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
const dialog = useDialog();

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

// 发布评论
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

// 初始化
const loadComments = async () => {
    const res = await fetchComments(articleId)

    comments.value = res.data.comments
}

// 删除评论
const handlerDeleteComment = async (commentid: string) => {
    dialog.warning({
        title: '确认删除?',
        content: '此操作将永久删除该评论，是否继续?',
        positiveText: '确定',
        negativeText: '取消',
        onPositiveClick: async () => {

            await DeleteComment(commentid)
            loadComments()
        }
    })
    // console.log(comments.value);
}

onMounted(() => {
    loadComments()
})

</script>
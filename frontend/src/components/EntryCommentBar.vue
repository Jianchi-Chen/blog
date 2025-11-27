<template>
    <n-alert title="提示" v-if="!userhasLogin">登录后才能发表评论</n-alert>
    <n-flex v-else>
        <n-form
            :model="formData"
            :rules="formRules"
            ref="formRef"
            class="w-[90%]"
        >
            <n-form-item path="newComment">
                <n-input
                    type="textarea"
                    :autosize="{
                        minRows: 2,
                        maxRows: 6,
                    }"
                    v-model:value="formData.newComment"
                    round
                    placeholder="feel free to feedback..."
                    bordered
                />
                <n-button
                    :loading="loading"
                    @click="submitComment"
                    round
                    type="primary"
                >
                    {{
                        commentCoolDown > 0
                            ? `${commentCoolDown}秒冷却`
                            : "发表评论"
                    }}
                </n-button>
            </n-form-item>
        </n-form>
    </n-flex>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import { inject, onMounted, onUnmounted, ref } from "vue";
import { useMessage, type FormInst } from "naive-ui";
import { postComment } from "@/api/comment";

const props = defineProps<{
    articleId: string;
}>();
const emit = defineEmits<{
    success: [];
}>();

const message = useMessage();
const userStore = useUserStore();
const userhasLogin = userStore.token ? true : false;
const newComment = ref();
const loading = ref(false);
const commentCoolDown = ref(0);
const timer = ref(0);
const formRef = ref<FormInst | null>();

const formData = ref({
    newComment: "",
});
const formRules = {
    newComment: {
        required: true,
        min: 2,
        message: "最短评论长度为 2",
    },
};

// 发布评论
const submitComment = async () => {
    loading.value = true;
    try {
        await formRef.value?.validate();
        await postComment(
            props.articleId,
            formData.value.newComment,
            userStore.username
        );
        message.success("评论成功");
        emit("success");
    } catch (e) {
        if (e instanceof Error) {
            message.error(`${e}`);
        }
    } finally {
        // 开始倒计时
        commentCoolDown.value = 3;
        timer.value = setInterval(() => {
            commentCoolDown.value--;
            if (commentCoolDown.value <= 0) {
                clearInterval(timer.value as number);
            }
        }, 1000);

        formData.value.newComment = "";
        setTimeout(() => {
            loading.value = false;
        }, 3000);
    }
};
</script>

<style scoped></style>

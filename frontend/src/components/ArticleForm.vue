<template>
    <div>
        <n-form :model="form" :rules="rules" ref="formRef" label-placement="top">
            <n-form-item label="标题" path="title">
                <n-input v-model:value="form.title" round placeholder="请输入文章标题" />
            </n-form-item>

            <n-form-item label="内容" path="content">
                <n-input v-model:value="form.content" type="textarea" placeholder="请输入文章内容(支持Markdown)"></n-input>
            </n-form-item>

            <n-form-item>
                <n-button type="primary" @click="handleSubmit" :loading="loading">{{ isEdit ? '保存修改' : '发布文章'
                    }}</n-button>
            </n-form-item>
        </n-form>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { NForm, NFormItem, NInput, NButton, useMessage } from 'naive-ui';
import axios from '@/api/client';
import { useRouter } from 'vue-router';
import { publishArticle, updateArticle } from '@/api/article';

// props类型约束
const props = defineProps<{
    article?: {
        title: string,
        content: string,
    }
    isEdit?: boolean,
    articleId?: number,
}>()

// emit
const emit = defineEmits<{
    (e: 'done'): void
}>()

// 表单数据
const form = ref({
    title: '',
    content: '',
})
// 校验规则
const rules = {
    title: {
        required: true, // 表示该字段是必填项
        message: '请输入标题', // 当用户未填写时显示的提示信息
        trigger: ['input', 'blur'],  // 表示在“输入”和“失焦”时触发验证
    },
    content: {
        required: true,
        message: '请输入内容',
        trigger: ['input', 'blur']
    }
}
const message = useMessage()
const router = useRouter();
const formRef = ref()
const loading = ref(false)

// 初始化表单内容并监听字段
watch(
    () => props.article,
    (val) => {
        if (val) form.value = { ...val }
    },
    { immediate: true }
)

// 提交表单
const handleSubmit = async () => {
    try {
        // naive ui 自带的表单校验
        await formRef.value?.validate()
        loading.value = true;

        if (props.isEdit && props.articleId) {
            // 编辑模式
            await updateArticle(props.articleId, form.value)
            message.success('Changed Success')
        } else {
            // 发布模式
            await publishArticle({ ...form.value, created_at: new Date().toISOString() })
        }

        emit('done')
        router.push('/admin')

    } catch (e) {
        console.error(e);
        message.error('发布失败');
    } finally {
        loading.value = false;
    }
}
</script>
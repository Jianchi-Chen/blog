<template>
    <h1>新建文章</h1>
    <ArticleForm :isEdit="false" :article="article" />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { NForm, NFormItem, NInput, NButton, useMessage } from 'naive-ui';
import axios from '@/api/client';
import { useRouter } from 'vue-router';
import ArticleForm from '@/components/ArticleForm.vue';
import type { Article } from '@/types/article'

const message = useMessage()
const router = useRouter()
const formRef = ref()

const article = ref({
    title: '',
    content: '',
})


</script>
<template>
    <h1>新建文章</h1>
    <ArticleForm :isEdit="false" :article="article" />
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from 'vue';
import { NForm, NFormItem, NInput, NButton, useMessage } from 'naive-ui';
import axios from '@/api/client';
import { useRouter } from 'vue-router';
import ArticleForm from '@/components/ArticleForm.vue';
import { ArticleSchema, createEmptyArticle, type Article } from '@/types/article'


// 初始化一个空字段，partial生成一个所有属性都为可选项的副本。数据不兼容，弃用
// const article: Ref<Article> = ref(ArticleSchema.partial().parse({}))

// 初始化方法二
const article: Article = createEmptyArticle()

</script>

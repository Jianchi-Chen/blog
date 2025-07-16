<template>

    <div class="max-w-3xl mx-auto p-6">
        <div v-if="loading" class="text-gray-500">加载中</div>
        <div v-else-if="error" class="text-red-500">{{ error }}</div>

        <div v-else>
            <h1 class="text-3xl font-bold mb-4">{{ article.title }}</h1>
            <p class="text-3xl font-bold mb-4">{{ article.created_at }}</p>
            <div class="prose">{{ article.content }}</div>
        </div>
    </div>






</template>

<script setup lang="ts">
import { fetchArticleById } from '@/api/article';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const articleId = route.params.id as string;

// 和articles的常量数组article不同，这里的acticle是一个变量
const article = ref<any>(null);
const loading = ref(true);
const error = ref('');

const loadArticle = async () => {
    loading.value = true;
    try {
        const res = await fetchArticleById(articleId);
        article.value = res.data;
    } catch (err) {
        error.value = "无法加载文章详情";
    } finally {
        loading.value = false;
    }
}

onMounted(() => {
    loadArticle();
})
</script>
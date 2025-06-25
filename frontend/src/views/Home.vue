<template>
    <div class="max-w-4xl mx-auto p-6">
        <h1 class="text-3xl font-bold mb-6">文章列表</h1>
        <div v-if="loading" class="text-gray-500">加载中</div>
        <div v-else-if="error" class="text-red-500">{{ error }}</div>

        <div v-else class="space-y-6">
            <div v-for="article in articles" :key="article.id" @click="goToDetail(article.id)"
                class="p-6 bg-white rounded shadow cursor-pointer hover:bg-gray-50">
                <h2 class="text-xl font-semibold text-blue-600">{{ article.title }}</h2>
                <p class="text-sm text-gray-500 mb-2">发布时间: {{ article.created_at }}</p>
                <p class="text-gray-700">{{ article.summary }}</p>
            </div>
        </div>
    </div>


</template>

<script setup lang="ts">
import { fetchArticleById, fetchArticles } from '@/api/article';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

// 文章列表
const router = useRouter();
const articles = ref<any[]>([]);
const loading = ref(true);
const error = ref('');

// 获取文章列表
const loadArticles = async () => {
    loading.value = true;
    try {
        const res = await fetchArticles();
        articles.value = res.data
    } catch (err) {
        error.value = "无法加载文章"
    } finally {
        loading.value = false;
    }
}

// 页面加载时请求
onMounted(() => {
    loadArticles();
});

// 跳转到详情
const goToDetail = (id: number| string) => {
    router.push(`/article/${id}`);
}

</script>
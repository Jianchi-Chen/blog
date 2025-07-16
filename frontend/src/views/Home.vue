<template>
    <div class="max-w-4xl mx-auto p-6">
        <h1 class="text-3xl font-bold mb-6 text-green-600">文章列表</h1>
        <div v-if="loading" class="text-gray-500">加载中</div>

        <!-- naive-ui实现 -->
        <n-flex vertical size="large">
            <n-card v-for="article in articles" :key="article.id" :hoverable="true" :embedded="true"
                @click="goToDetail(article.id)" class="cursor-pointer">
                <template #header>
                    <h2 class="text-xl font-semibold text-blue-600">{{ article.title }}</h2>
                </template>

                <p class="text-sm text-gray-500 mb-2">发布时间: {{ article.created_at }}</p>
                <p class="text-gray-700">{{ article.summary }}</p>
            </n-card>
        </n-flex>
    </div>


</template>

<script setup lang="ts">
import { NCard, NFlex, useMessage } from 'naive-ui';
import { fetchArticleById, fetchArticles } from '@/api/article';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

// 文章列表
const router = useRouter();
const articles = ref<any[]>([]);
const loading = ref(true);
const message = useMessage();

// 获取文章列表
const loadArticles = async () => {
    loading.value = true;
    try {
        const res = await fetchArticles();
        articles.value = res.data
    } catch (err) {
        message.error('无法加载文章, 请刷新', {
            duration: 0, // 设置为 0 表示永不自动关闭
            closable: true, // 加一个关闭按钮以防无法关闭
        });
    } finally {
        loading.value = false;
    }
}

// 页面加载时请求
onMounted(() => {
    loadArticles();
});

// 跳转到详情
const goToDetail = (id: number | string) => {
    router.push(`/article/${id}`);
}

</script>
<template>
    <n-flex vertical size="medium">
        <!-- naive-ui实现 -->
        <n-h1 prefix="bar">
            <n-text type="success">
                文章列表
            </n-text>
        </n-h1>
        <n-h2 v-if="loading">加载中</n-h2>

        <n-card v-for="article in articles" :key="article.id" :hoverable="true" :embedded="true"
            @click="goToDetail(article.id)" class="cursor-pointer">
            <n-h2>
                <n-text type="info">
                    {{ article.title }}
                </n-text>
            </n-h2>

            <n-text depth="3">发布时间: {{ article.created_at }}</n-text>
            <n-p>{{ article.summary }}</n-p>
        </n-card>
    </n-flex>




</template>

<script setup lang="ts">
import { NCard, NFlex, useMessage, NH1, NText, NH2, NP, NLayout } from 'naive-ui';
import { fetchArticleById, fetchArticles } from '@/api/article';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useUserStore } from '@/stores/user';

// 文章列表
const router = useRouter();
const articles = ref<any[]>([]);
const loading = ref(true);
const message = useMessage();

// 获取文章列表
const loadArticles = async () => {
    loading.value = true;
    try {
        const res = await fetchArticles("vistor");
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
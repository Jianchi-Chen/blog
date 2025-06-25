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


    <!-- 测试prose渲染 -->
    <div class="prose mx-auto p-6">
        <h1>这是一级标题</h1>
        <p>这是一个正文段落，应该有良好的行距和适当的边距。</p>
        <h2>这是二级标题</h2>
        <ul>
            <li>第一个列表项</li>
            <li>第二个列表项</li>
        </ul>
        <blockquote>这是一段引用，应该自动带边框和内缩样式。</blockquote>
        <pre><code>console.log("这是代码块")</code></pre>
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
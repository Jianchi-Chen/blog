<template>
    <h1>编辑文章</h1>
    <div v-if="loading">加载中</div>
    <ArticleForm v-else :isEdit="true" :articleId="Number(route.params.id)" :article="article" />
</template>

<script setup lang="ts">
import { fetchArticleById } from '@/api/article';
import ArticleForm from '@/components/ArticleForm.vue';
import { ref } from 'vue';
import { onMounted } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute()
const article = ref()
const loading = ref(true)

const init = async () => {
    try {
        const res = await fetchArticleById(Number(route.params.id))
        console.log(`res内容:${res.data}`)
        article.value = res.data
    } catch (e) {
        console.error('加载失败', e)
    } finally {
        loading.value = false
    }

}

onMounted(init)
</script>
<template>
    <!-- todo 待施工 -->
    <n-flex>
        <n-text v-if="loading" class="text-gray-500">加载中</n-text>
        <n-text v-else-if="error" class="text-red-500">{{ error }}</n-text>

        <n-text v-else>
            <n-h1>{{ article.title }}</n-h1>
            <n-p>创建于 {{ article.created_at }}</n-p>
            <!-- <div class="prose">{{ article.content }}</div> -->

            <div>
                <!-- vue3 自解包ref，所以不能加.value -->
                <MdPreview :md="article.content" />
            </div>
            <n-hr />
            <div>
                <CommentSection />
            </div>

        </n-text>
    </n-flex>

</template>

<script setup lang="ts">
import { fetchArticleById } from '@/api/article';
import CommentSection from '@/components/CommentSection.vue';
import MdPreview from '@/components/MdPreview.vue';
import { ArticleSchema, createEmptyArticle, type Article } from '@/types/article';
import { onMounted, ref, type Ref } from 'vue';
import { useRoute } from 'vue-router';
import { NCard, NFlex, useMessage, NH1, NText, NH2, NP, NLayout, NHr } from 'naive-ui';

const route = useRoute();
const articleId = route.params.id as string;

// 和articles的常量数组article不同，这里的acticle是一个变量
const article: Ref<Article> = ref(createEmptyArticle());
const loading = ref(true);
const error = ref('');

const loadArticle = async () => {
    loading.value = true;
    try {
        const res = await fetchArticleById(articleId);
        if (res.data) {
            article.value = { ...res.data };
        }
        // console.log(article.value.content)
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
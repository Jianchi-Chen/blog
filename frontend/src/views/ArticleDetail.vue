<template>
    <n-flex size="small" style="width: 100%">
        <n-text v-if="loading" class="text-gray-500">
            <n-skeleton text :repeat="2" />
            <n-skeleton text style="width: 60%" />
        </n-text>
        <n-text v-else-if="error" class="text-red-500">{{ error }}</n-text>

        <n-text v-else class="w-[90%]">
            <n-h1>{{ article.title }}</n-h1>
            <n-h3>创建于 {{ article.created_at }}</n-h3>
            <n-flex size="medium">
                <n-h3>文章标签: </n-h3>
                <!-- <n-tag v-for="(tag, index) in tags" :key="index" type="success">{{ tag }} </n-tag> 当tags是数组时 -->
                <n-tag type="success">{{ tags }} </n-tag>

                <n-hr></n-hr>
            </n-flex>

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
import { fetchArticleById } from "@/api/article";
import CommentSection from "@/components/CommentSection.vue";
import MdPreview from "@/components/MdPreview.vue";
import {
    ArticleSchema,
    createEmptyArticle,
    type Article,
} from "@/types/article";
import { computed, onMounted, ref, watchEffect, type Ref } from "vue";
import { useRoute } from "vue-router";
import {
    NCard,
    NFlex,
    useMessage,
    NH1,
    NText,
    NH2,
    NH3,
    NP,
    NLayout,
    NHr,
    NDynamicTags,
    NTag,
} from "naive-ui";
import { ArrowUpCircle } from "@vicons/ionicons5";

const route = useRoute();
const articleId = computed(() => route.params.id as string);

// 和articles的常量数组article不同，这里的acticle是一个变量
const article: Ref<Article> = ref(createEmptyArticle());
const loading = ref(false);
const error = ref("");
const tags: Ref<Article["tags"]> = ref("");

const loadArticle = async () => {
    loading.value = true;
    try {
        const res = await fetchArticleById(articleId.value);
        if (res.data) {
            tags.value = res.data.tags;
            article.value = { ...res.data };
        }
        // console.log(article.value.title)
    } catch (err) {
        error.value = "无法加载文章详情";
    } finally {
        loading.value = false;
    }
};

onMounted(() => {
    loadArticle();
});

watchEffect(() => {
    if (!articleId.value) return;
    else loadArticle();
});
</script>

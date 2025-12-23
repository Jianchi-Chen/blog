<template>
    <n-space vertical size="large" style="width: 100%">
        <n-card bordered size="large">
            <template #header>
                <div class="flex items-start justify-between w-full">
                    <div class="flex-1">
                        <h1 class="text-2xl font-semibold leading-tight">
                            {{ article.title }}
                        </h1>
                        <div
                            class="mt-2 flex items-center gap-3 text-sm text-gray-500"
                        >
                            <n-avatar :size="28">{{
                                (article.author_name || "C")
                                    .charAt(0)
                                    .toUpperCase()
                            }}</n-avatar>
                            <span class="truncate">{{
                                article.author_name || "匿名"
                            }}</span>
                            <n-time
                                :value="article.created_at"
                                time-zone="Asia/Shanghai"
                                format="yyyy-MM-dd hh:mm"
                            />
                            <n-tag type="success" size="small">{{
                                tags || "未分类"
                            }}</n-tag>
                        </div>
                    </div>
                </div>
            </template>

            <template #default>
                <p class="text-gray-600 my-4">
                    {{ article.summary || "暂无摘要" }}
                </p>
                <n-divider />

                <div class="prose max-w-none my-4">
                    <!-- 保持 MdPreview 不变 -->
                    <MdPreview :md="article.content" />
                </div>
            </template>
        </n-card>

        <n-card bordered>
            <!-- 保持 CommentSection 不变 -->
            <CommentSection />
        </n-card>
    </n-space>
</template>

<script setup lang="ts">
import { fetchArticleById } from "@/api/article";
import CommentSection from "@/components/article/CommentSection.vue";
import MdPreview from "@/components/article/MdPreview.vue";
import {
    ArticleSchema,
    createEmptyArticle,
    type Article,
} from "@/types/article";
import { computed, onMounted, ref, watchEffect, type Ref } from "vue";
import { useRoute } from "vue-router";
import { NCard, NSpace, NAvatar, NTime, NTag, NDivider } from "naive-ui";
import { ArrowUpCircle } from "@vicons/ionicons5";

const route = useRoute();
const articleId = computed(() => route.params.id as string);

// article 是一个可变的响应式对象
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
    loadArticle();
});
</script>

<template>
    <n-layout class="min-h-full">
        <n-layout-content class="px-6 py-8 max-w-5xl mx-auto">
            <n-space vertical :size="32">
                <!-- 页面标题与筛选区域 -->
                <n-space vertical :size="16">
                    <n-h1
                        class="!mb-0"
                        :style="{ fontSize: '2rem', fontWeight: 600 }"
                    >
                        <n-gradient-text type="success">
                            文章列表
                        </n-gradient-text>
                    </n-h1>

                    <!-- 筛选工具栏 -->
                    <n-space align="center" :size="12">
                        <n-popselect
                            v-model:value="select_value"
                            multiple
                            :options="select_options"
                            trigger="click"
                        >
                            <n-button
                                :type="
                                    select_value.length ? 'primary' : 'default'
                                "
                                secondary
                            >
                                <template #icon>
                                    <n-icon>
                                        <FilterOutline />
                                    </n-icon>
                                </template>
                                {{
                                    Array.isArray(select_value) &&
                                    select_value.length
                                        ? `已选 ${select_value.length} 个标签`
                                        : "筛选标签"
                                }}
                            </n-button>
                        </n-popselect>

                        <n-tag
                            v-for="tag in select_value"
                            :key="tag"
                            :closable="true"
                            type="success"
                            @close="removeTag(tag)"
                            size="medium"
                        >
                            {{ tag }}
                        </n-tag>

                        <n-button
                            v-if="select_value.length"
                            text
                            @click="clearSelectedTags"
                            type="error"
                        >
                            <template #icon>
                                <n-icon>
                                    <TrashOutline />
                                </n-icon>
                            </template>
                            清空
                        </n-button>
                    </n-space>
                </n-space>

                <!-- 文章列表 -->
                <n-spin :show="loading" size="large">
                    <n-empty
                        v-if="!loading && articles.length === 0"
                        description="暂无文章"
                        size="large"
                        class="py-12"
                    />

                    <n-space vertical :size="16" v-else>
                        <n-card
                            v-for="article in articles"
                            :key="article.id"
                            :hoverable="true"
                            @click="goToDetail(article.id)"
                            class="cursor-pointer transition-all"
                            :bordered="false"
                            :segmented="{
                                content: 'soft',
                                footer: 'soft',
                            }"
                        >
                            <!-- 顶栏：标题 -->
                            <template #header>
                                <n-ellipsis
                                    :line-clamp="2"
                                    :style="{
                                        fontSize: '1.25rem',
                                        fontWeight: 600,
                                    }"
                                >
                                    {{ article.title }}
                                </n-ellipsis>
                            </template>

                            <!-- 中间:文章摘要 -->
                            <n-ellipsis
                                :line-clamp="3"
                                class="text-base opacity-80"
                            >
                                {{ article.summary || "暂无摘要" }}
                            </n-ellipsis>

                            <!-- 底栏: 文章创建时间和标签 -->
                            <template #footer>
                                <n-space align="center" justify="space-between">
                                    <n-space align="center" :size="8">
                                        <n-icon size="16" class="opacity-60">
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                viewBox="0 0 24 24"
                                            >
                                                <path
                                                    fill="currentColor"
                                                    d="M19 4h-1V2h-2v2H8V2H6v2H5c-1.11 0-1.99.9-1.99 2L3 20a2 2 0 0 0 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H5V10h14v10zM9 14H7v-2h2v2zm4 0h-2v-2h2v2zm4 0h-2v-2h2v2zm-8 4H7v-2h2v2zm4 0h-2v-2h2v2zm4 0h-2v-2h2v2z"
                                                />
                                            </svg>
                                        </n-icon>
                                        <n-text
                                            depth="3"
                                            :style="{ fontSize: '0.875rem' }"
                                        >
                                            {{ formatDate(article.created_at) }}
                                        </n-text>
                                    </n-space>

                                    <n-tag
                                        v-if="article.tags"
                                        type="info"
                                        size="small"
                                        :bordered="false"
                                    >
                                        {{ article.tags }}
                                    </n-tag>
                                </n-space>
                            </template>
                        </n-card>
                    </n-space>
                </n-spin>
            </n-space>
        </n-layout-content>
    </n-layout>
</template>

<script setup lang="ts">
import {
    NCard,
    NSpace,
    useMessage,
    NH1,
    NText,
    NTag,
    NEmpty,
    NSpin,
    NGradientText,
    NEllipsis,
    NIcon,
    NButton,
    NPopselect,
    NLayout,
    NLayoutContent,
} from "naive-ui";
import { fetchArticles } from "@/api/article";
import { onMounted, ref, watch, type Ref } from "vue";
import { useRouter } from "vue-router";
import type { Article } from "@/types/article";
import { useSearchStore } from "@/stores/search";
import { FilterOutline, TrashOutline } from "@vicons/ionicons5";

// 文章列表
const router = useRouter();
const articles = ref<any[]>([]);
const loading = ref(true);
const message = useMessage();
const select_value = ref<string[]>([]);
const search = useSearchStore();

// 筛选项，需要指明对象内容，不然不给value
import type { SelectOption } from "naive-ui";

const select_options: Ref<SelectOption[]> = ref([]);

// 获取文章列表
const loadArticles = async () => {
    loading.value = true;
    try {
        const res = await fetchArticles("vistor", search.condition);
        articles.value = res.data.articles; // default is a object

        // console.log(articles.value);

        getTags();
    } catch (err) {
        message.error("无法加载文章, 请刷新", {
            duration: 0, // 设置为 0 表示永不自动关闭
            closable: true, // 加一个关闭按钮以防无法关闭
        });
    } finally {
        loading.value = false;
    }
};

// 页面加载时请求
onMounted(() => {
    loadArticles();
});

// 跳转到详情
const goToDetail = (id: number | string) => {
    router.push(`/article/${id}`);
};

// 获取并且更新标签
const getTags = () => {
    // copilot优化，获取tag
    const tagSet = new Set<string>();

    for (const i of articles.value) {
        tagSet.add(i.tags);
    }
    // sort()升序，map()将每一个tag字符串转换为一个对象
    select_options.value = Array.from(tagSet)
        .sort()
        .map((tag) => ({
            label: tag,
            value: tag,
        }));
};

// 根据选中标签过滤文章（不在函数内修改 `select_value`，避免触发循环）
const applyTagFilter = (selected: string[] | undefined) => {
    const sel = Array.isArray(selected) ? selected.map(String) : [];

    if (!sel.length) {
        // 未选中任何标签，恢复完整列表
        loadArticles();
        return;
    }

    const newArticles: Article[] = articles.value.filter((a: any) =>
        sel.includes(String(a.tags))
    );

    articles.value = newArticles;
    console.log("筛选后的文章列表: ", articles.value);
    getTags();
};

// 清空所选标签
const clearSelectedTags = () => {
    select_value.value = [];
};

// 移除单个标签
const removeTag = (tag: string) => {
    select_value.value = select_value.value.filter((t) => t !== tag);
};

// 格式化日期
const formatDate = (dateStr: string) => {
    if (!dateStr) return "";
    const date = new Date(dateStr);
    return date.toLocaleDateString("zh-CN", {
        year: "numeric",
        month: "long",
        day: "numeric",
    });
};

// 监听 select_value 的变化来触发筛选（使用 v-model 触发）
watch(select_value, (newVal) => {
    applyTagFilter(newVal as string[]);
});

// 仅在搜索条件变更时重新加载文章（避免无条件重复触发）
watch(
    () => search.condition,
    () => {
        loadArticles();
    }
);
</script>

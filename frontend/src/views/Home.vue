<template>
    <!-- naive-ui实现 -->
    <n-flex vertical size="medium">
        <!-- 标签过滤 -->
        <n-flex align="center">
            <n-h1 prefix="bar">
                <n-text type="success"> 文章列表 </n-text>
            </n-h1>
            <n-popselect
                v-model:value="select_value"
                multiple
                :options="select_options"
            >
                <n-button :type="select_value.length ? 'success' : 'default'">
                    <n-icon size="20">
                        <FilterOutline />
                    </n-icon>
                    {{
                        Array.isArray(select_value) && select_value.length
                            ? `已选标签：${select_value.join(", ")}`
                            : "按标签选择"
                    }}
                </n-button>
            </n-popselect>
            <n-button @click="clearSelectedTags">
                <n-icon size="20">
                    <TrashOutline />
                </n-icon>
                清空所选标签</n-button
            >
        </n-flex>

        <!-- 文章卡片展示 -->
        <n-h2 v-if="loading">加载中</n-h2>
        <n-card
            v-for="article in articles"
            :key="article.id"
            :hoverable="true"
            :embedded="true"
            @click="goToDetail(article.id)"
            class="cursor-pointer"
        >
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
import {
    NCard,
    NFlex,
    useMessage,
    NH1,
    NText,
    NH2,
    NP,
    NLayout,
} from "naive-ui";
import {
    fetchArticleByConditions,
    fetchArticleById,
    fetchArticles,
} from "@/api/article";
import { onMounted, ref, watch, type Ref } from "vue";
import { useRouter } from "vue-router";
import { useUserStore } from "@/stores/user";
import { array, optional } from "zod";
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
interface Option {
    label: string;
    value: string | number;
}
const select_options: Ref<Option[]> = ref([]);

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

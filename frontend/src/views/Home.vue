<template>
    <!-- naive-ui实现 -->
    <n-flex vertical size="medium">
        <!-- 标签过滤 -->
        <n-flex align="center">
            <n-h1 prefix="bar">
                <n-text type="success">
                    文章列表
                </n-text>
            </n-h1>
            <n-popselect v-model:value="select_value" multiple :options="select_options"
                :on-update:value="handlePopSelect">
                <n-button :type="select_value.length ? 'success' : 'default'">
                    {{ Array.isArray(select_value) && select_value.length ? `已选标签：${select_value.join(', ')}` : '按标签选择'
                    }}
                </n-button>
            </n-popselect>
            <n-button @click="loadArticles">清空所选标签</n-button>
        </n-flex>

        <!-- 文章卡片展示 -->
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
import { fetchArticleByConditions, fetchArticleById, fetchArticles } from '@/api/article';
import { onMounted, ref, watch, watchEffect, type Ref } from 'vue';
import { useRouter } from 'vue-router';
import { useUserStore } from '@/stores/user';
import { array, optional } from 'zod';
import type { Article } from '@/types/article';
import type { SelectBaseOption } from 'naive-ui/es/select/src/interface';
import { useSearchStore } from '@/stores/search';

// 文章列表
const router = useRouter();
const articles = ref<any[]>([]);
const loading = ref(true);
const message = useMessage();
const select_value = ref<string[]>([])
const search = useSearchStore()

// 筛选项，需要指明对象内容，不然不给value
interface Option {
    label: string
    value: string | number
}
const select_options: Ref<Option[]> = ref([])

// 获取文章列表
const loadArticles = async () => {
    loading.value = true;
    try {
        const res = await fetchArticles("vistor", search.condition);
        articles.value = res.data

        getTags()
        select_value.value = []

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

// 获取并且更新标签
const getTags = () => {
    // copilot优化，获取tag
    const tagSet = new Set<string>()

    for (const i of articles.value) {
        for (const tag of i.tags) {
            tagSet.add(tag)
        }
    }
    // sort()升序，map()将每一个tag字符串转换为一个对象
    select_options.value = Array.from(tagSet).sort().map(tag => ({
        label: tag,
        value: tag
    }))
}

// 处理标签更新
const handlePopSelect = (value: string | number | Array<string | number>, option: SelectBaseOption | null | Array<SelectBaseOption>) => {

    const newArticles: Article[] = []
    for (const i of articles.value) {
        for (const tag of i.tags) {
            if (Array.isArray(value) && value.includes(tag)) {
                newArticles.push(i)
                select_value.value = value as string[]

                break
            } else if (value === tag) {
                newArticles.push(i)
                select_value.value.push(String(value))

                break
            }
        }
    }
    articles.value = newArticles
    getTags()

}

watchEffect(() => {
    console.log('当前关键词：', search.condition)
    loadArticles()
})

</script>
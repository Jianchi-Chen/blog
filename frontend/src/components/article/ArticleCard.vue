<template>
    <n-card
        :hoverable="true"
        @click="handleClick"
        class="cursor-pointer transition-all"
        :bordered="false"
        :segmented="{
            content: 'soft',
            footer: 'soft',
        }"
    >
        <!-- 顶栏：标题 -->
        <template #header>
            <n-ellipsis :line-clamp="2" :style="{ fontSize: '1.25rem', fontWeight: 600 }">
                {{ article.title }}
            </n-ellipsis>
        </template>

        <!-- 中间:文章摘要 -->
        <n-ellipsis :line-clamp="1" class="text-base opacity-80">
            {{ article.summary || "暂无摘要" }}
        </n-ellipsis>

        <!-- 底栏: 文章创建时间和标签 -->
        <template #footer>
            <n-space align="center" justify="space-between">
                <n-space align="center" :size="8">
                    <n-icon size="16" class="opacity-60">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                            <path
                                fill="currentColor"
                                d="M19 4h-1V2h-2v2H8V2H6v2H5c-1.11 0-1.99.9-1.99 2L3 20a2 2 0 0 0 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H5V10h14v10zM9 14H7v-2h2v2zm4 0h-2v-2h2v2zm4 0h-2v-2h2v2zm-8 4H7v-2h2v2zm4 0h-2v-2h2v2zm4 0h-2v-2h2v2z"
                            />
                        </svg>
                    </n-icon>
                    <n-text depth="3" :style="{ fontSize: '0.875rem' }">
                        {{ formattedDate }}
                    </n-text>
                </n-space>

                <n-tag v-if="article.tags" type="info" size="small" :bordered="false">
                    {{ article.tags }}
                </n-tag>
            </n-space>
        </template>
    </n-card>
</template>

<script setup lang="ts">
import type { Article } from "@/types/article";
import { computed } from "vue";

const props = defineProps<{
    article: Article;
}>();

const emit = defineEmits<{
    click: [id: string | number];
}>();

const handleClick = () => {
    emit("click", props.article.id!);
};

// 格式化日期
const formattedDate = computed(() => {
    if (!props.article.created_at) return "";

    // 处理 "2025::11::25" 格式，将 :: 替换为 -
    const normalizedDate = props.article.created_at.replace(/::/g, "-");
    const date = new Date(normalizedDate);

    // 检查日期是否有效
    if (isNaN(date.getTime())) {
        return props.article.created_at; // 如果无法解析，返回原始字符串
    }

    return date.toLocaleDateString("zh-CN", {
        year: "numeric",
        month: "long",
        day: "numeric",
    });
});
</script>

<template>
    <n-tag :type="statusMap[status]?.type"> {{ statusMap[status]?.label }}</n-tag>
</template>

<script setup lang="ts">
import { NTag } from 'naive-ui';
// 引入 Naive UI 提供的 Tag 类型
import type { TagProps } from 'naive-ui';

defineProps<{ status: string }>()

// 把 type 的类型写成 TagProps['type']
const statusMap: Record<string, { label: string, type: TagProps['type'] }> = {
    published: { label: "✅ 已发布", type: 'success' },
    draft: { label: "📝 草稿", type: 'warning' },
    archived: { label: "📦 已下架", type: 'error' }
}
</script>
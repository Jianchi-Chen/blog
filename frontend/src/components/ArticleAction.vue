<template>
    <n-button strong tertiary type="info" @click="$emit('edit', id)">编辑</n-button>
    <n-button strong tertiary :type="statusMap[status]?.type" @click="handleToggleStatus">{{ statusMap[status]?.label
        }}</n-button>
    <n-button strong tertiary type="error" @click="$emit('delete', id)">删除</n-button>
</template>

<script setup lang="ts">
import { NButton, type TagProps } from 'naive-ui';

// id为主键, status为文章状态
const { id, status } = defineProps<{ id: number, status: string }>();
const emit = defineEmits<{
    (e: 'edit', id: number): void,
    (e: 'delete', id: number): void,
    (e: 'toggleStatus', id: number, toggle: string): void,
}>();

const statusMap: Record<string, { label: string, type: TagProps['type'] }> = {
    published: { label: "归档", type: "default" },
    archived: { label: "发布", type: "primary" },
    draft: { label: "发布", type: "primary" },
}

const handleToggleStatus = () => {
    // @click
    if (status === 'published') {
        emit('toggleStatus', id, 'archived');
    } else if (status === 'archived') {
        emit('toggleStatus', id, 'published');
    } else {
        emit('toggleStatus', id, 'published');
    }

    // :type

}
</script>

<!-- 编辑后变为草稿状态 -->
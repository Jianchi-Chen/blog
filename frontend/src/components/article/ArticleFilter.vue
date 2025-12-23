<template>
    <n-space align="center" :size="12">
        <n-popselect
            v-model:value="selectedTags"
            multiple
            :options="tagOptions"
            trigger="click"
        >
            <n-button
                :type="selectedTags.length ? 'primary' : 'default'"
                secondary
            >
                <template #icon>
                    <n-icon>
                        <FilterOutline />
                    </n-icon>
                </template>
                {{
                    Array.isArray(selectedTags) && selectedTags.length
                        ? `已选 ${selectedTags.length} 个标签`
                        : "筛选标签"
                }}
            </n-button>
        </n-popselect>

        <n-tag
            v-for="tag in selectedTags"
            :key="tag"
            :closable="true"
            type="success"
            @close="handleRemoveTag(tag)"
            size="medium"
        >
            {{ tag }}
        </n-tag>

        <n-button
            v-if="selectedTags.length"
            text
            @click="handleClearTags"
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
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { SelectOption } from "naive-ui";
import { FilterOutline, TrashOutline } from "@vicons/ionicons5";

const props = defineProps<{
    modelValue: string[];
    tagOptions: SelectOption[];
}>();

const emit = defineEmits<{
    "update:modelValue": [value: string[]];
}>();

const selectedTags = computed({
    get: () => props.modelValue,
    set: (value: string[]) => emit("update:modelValue", value),
});

const handleRemoveTag = (tag: string) => {
    const newTags = props.modelValue.filter((t) => t !== tag);
    emit("update:modelValue", newTags);
};

const handleClearTags = () => {
    emit("update:modelValue", []);
};
</script>

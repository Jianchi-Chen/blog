<template>
    <div>
        <div class="vditor-preview" ref="vditorRef"></div>
    </div>
</template>


<script setup lang="ts">
import { useArticleStore } from '@/stores/article';
import type { Article } from '@/types/article';
import Vditor from 'vditor';
import 'vditor/dist/index.css' // vditor-preview模式
import { nextTick, onMounted, ref, watch, watchEffect } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute()
const articleStore = useArticleStore()
const id = route.params.id as Article["id"]
const vditorRef = ref<HTMLDivElement>()
const props = defineProps<{
    md: Article['content']
}>()
const emit = defineEmits<{
    (e: 'done'): void
}>()

const theme = ref("classic")

// console.log(vditorRef)
// 渲染函数；典范代码
const render = (markdown: string | undefined) => {

    // 等 DOM 更新完成后再执行你的代码。
    nextTick(() => {
        if (vditorRef.value && markdown !== undefined) {
            // 直接调用静态方法渲染

            Vditor.preview(
                vditorRef.value,
                markdown,
                {
                    mode: 'light',
                    // 更多配置见官方文档
                    theme: {
                        current: theme.value
                    },
                }
            )
        }
        // console.log("MdPreview.vue: ");
        // console.log(theme.value, articleStore.osTheme);
    })

}
// 新值会作为第一个参数传给render
// watch(() => props.md, render, { immediate: true })
watchEffect(() => {
    if (articleStore.osTheme || props.md) {
        // console.log("1");
        theme.value = articleStore.osTheme === false ? "light" : "dark";
        render(props.md);
    }
})

</script>
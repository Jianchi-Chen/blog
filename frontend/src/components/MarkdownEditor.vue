<template>
    <div>
        <!-- vditor需要一个“干净”的DOM节点，可用div或textarea -->
        <div :id="vditorId" class="vditor"></div>
    </div>

</template>

<script setup lang="ts">
import type { Article } from '@/types/article';
import { uuidv7 } from 'uuidv7';
import Vditor from 'vditor';
import { onBeforeUnmount, onMounted, ref, watch, type Ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';


const props = defineProps<{
    // vditor的内容
    modelValue: string | undefined,
    // articleId: string,
}>()
const emit = defineEmits<{
    // 当modelValue更新时，接收val值，并不返回任何值。父组件使用:model接收
    // (e: `update:modelValue`, val: string): void
    'update:modelValue': [val: string]
}>()

const route = useRoute()
const id = route.params.id as Article["id"]
const vditorId: string = id || uuidv7() // 不用id的话没法开启vditor的缓存功能，使用uuidv7()生成每个编辑器的唯一id
const vditor = ref<Vditor>()


// 初始化vditor编辑器
onMounted(() => {
    // 如果没有值，则初始化为''
    const initialValue = props.modelValue || '';
    console.log(initialValue)
    // if起到断言不为undefined的作用
    if (vditorId) {
        console.log(vditorId)
        vditor.value = new Vditor(vditorId, {
            height: 400,
            value: initialValue,
            placeholder: '请输入Markdown内容...',
            mode: 'sv',
            preview: {
                markdown: {
                    toc: true,
                    mark: true,
                    footnotes: true,
                    // autoSpace: true
                }
            },
            input(value) {
                emit(`update:modelValue`, value)
            },
            // cache: { enable: false },      // 防止缓存覆盖

        })
    }
})

// 监听外部传入的 modelValue 变化，并同步更新 Vditor 编辑器的内容
watch(() => props.modelValue,
    (val) => {
        if (vditor.value && vditor.value.getValue() !== val && val) {
            vditor.value.setValue(val)
        }
    },
    { immediate: true }
)

// 离开时销毁
onBeforeUnmount(() => {
    vditor.value?.destroy()
})
</script>
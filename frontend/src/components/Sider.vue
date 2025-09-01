<template>
    <n-menu :options="menuOptions" @update:value="handleUpdateValue" :collapsed="articleStore.expandFolder"
        :collapsed-icon-size="22" :collapsed-width="64" :render-icon="renderMenuIcon" />
</template>

<script setup lang="ts">
import { NEllipsis, NIcon, useMessage, type MenuOption } from 'naive-ui';
import { computed, h, onMounted, ref, watchEffect, type Component, type Ref } from 'vue';
import { RouterLink, useRouter } from 'vue-router';
import {
    BookOutline as BookIcon,
    HomeOutline as HomeIcon,
    PersonOutline as PersonIcon,
    WineOutline as WineIcon
} from '@vicons/ionicons5'
import { fetchArticles } from '@/api/article';
import type { any } from 'zod';
import { useArticleStore } from '@/stores/article';

const mes = useMessage()
const router = useRouter()
const articleStore = useArticleStore()
// const anti_collision_array = [""]
const renderIcon = (icon: Component) => {
    return () => h(NIcon, null, { default: () => h(icon) })
}

// goHome
const goHomeMenuOptions = {
    label: 'Home',
    key: '/',
    // icon: renderIcon(HomeIcon)
}
// 线
const hrMenuOptions = {
    key: 'divider-1',
    type: 'divider',
    props: {
        style: {
            marginLeft: '32px'
        }
    }
}

const menuOptions: Ref<MenuOption[]> = ref([
    goHomeMenuOptions,
    hrMenuOptions,
])

// router-link 
const handleUpdateValue = (key: string, item: MenuOption) => {
    if (key === '/') {
        router.push('/')
        return
    }
    router.push(`/article/${key}`)
}

const loadMenu = async () => {
    try {
        menuOptions.value = [goHomeMenuOptions,
            hrMenuOptions]
        // console.log(menuOptions.value);

        const res = await fetchArticles("vistor")
        if (Array.isArray(res.data.articles)) {
            const tagMap = new Map<string, any>() // 用于快速查找已有标签项
            res.data.articles.forEach((i: any) => {
                const tagKey = i.tags || "Universal" // 空标签归为 Universal

                if (tagMap.has(tagKey)) {
                    tagMap.get(tagKey).children.push({
                        label: i.title,
                        key: i.id,
                    })
                } else {
                    const labelContent = tagKey === "Universal" ? "Universal" : () => h(NEllipsis, null, { default: () => tagKey })

                    const newgroup = {
                        key: tagKey + '_group',
                        label: labelContent,
                        children: [
                            {
                                label: i.title,
                                key: i.id,
                            }
                        ]
                    }

                    tagMap.set(tagKey, newgroup)
                }
            })
            menuOptions.value = Array.from(tagMap.values())
        }

        // console.log("当前menuOptions: ", menuOptions);
    }
    catch (e) {
        if (e instanceof Error) {
            mes.error(`发生错误：${e.message}`, {
                closable: true, // 手动关闭
                duration: 5000
            })
        }
    }
}

onMounted(() => {
    loadMenu()
})

watchEffect(() => {
    // 更新侧边栏内容
    if (articleStore.updateFolderContentSignal) {
        loadMenu()
        articleStore.updateFolderContentSignal = false
    }
})

// 批量渲染图标
const renderMenuIcon = (option: MenuOption) => {
    /** 
        // 渲染图标占位符以保持缩进
        if (option.key === 'sheep-man')
            return true
        // 返回 falsy 值，不再渲染图标及占位符
        if (option.key === 'food')
            return null
    */
    if (option.key === '/')
        return h(NIcon, null, { default: () => h(HomeIcon) })
    console.log("hahahsdhasd");
    if (option.children) {
        return h(NIcon, null, { default: () => h(BookIcon) })
    }
    return null
}

</script>
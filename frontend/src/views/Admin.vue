<template>
    <n-flex vertical :size="12">
        <n-flex>
            <n-button type="primary" @click="handleAdd">New Article</n-button>
            <n-button @click="clearSorter">
                Clear Sorter
            </n-button>
        </n-flex>

        <!-- 标签页 -->
        <n-card>
            <n-tabs>
                <n-tab-pane name="articleAdmin" tab="文章管理">
                    <n-data-table :columns="columnsRef" :data="data" :pagination="pagination" :bordered="true"
                        ref="tableRef" @update:sorter="handleSorterChange"></n-data-table>
                </n-tab-pane>

                <!-- 目录管理暂时没想到有什么用处，故不保留 -->
                <!-- <n-tab-pane name="dynamicTagsManager" tab="目录管理">
                    <DynamicTagsManager />
                </n-tab-pane> -->
            </n-tabs>
        </n-card>
    </n-flex>
</template>

<script setup lang="ts">
import { NDataTable, NButton, useDialog, useMessage } from 'naive-ui';
import type { DataTableSortOrder, DataTableBaseColumn, DataTableColumns, DataTableSortState } from 'naive-ui'
import { deleteArticle, fetchArticleById, fetchArticles, toggleStatus } from '@/api/article';
import { useUserStore } from '@/stores/user';
import { computed, h, onMounted, reactive, ref, render, watchEffect } from 'vue';
import { useRouter } from 'vue-router';
import axios from '@/api/client';
import StatusTag from '@/components/StatusTag.vue';
import ArticleAction from '@/components/ArticleAction.vue';
import type { Article } from '@/types/article';
import type { EnumType } from 'typescript';
import { array } from 'zod';
import { useArticleStore } from '@/stores/article';


const loading = ref(true);
const router = useRouter();
const userStore = useUserStore();
const dialog = useDialog();
const message = useMessage();
const data = ref<Article[]>([]);
const tableRef = ref();
const articleStore = useArticleStore()

interface RowData {
    id: string,
    key: string,
    title: string,
    created_at: string,
    status: Article["status"],
    tags: string[]
}
// 收集所有tag，返回string[]
const allTags = computed(() => {
    const set = new Set<string>()
    data.value.forEach(i => {
        set.add(i.tags);
    })
    return Array.from(set)
})
// 标签筛选
const titleColumn: DataTableBaseColumn<RowData> = {
    title: '标题',
    key: 'title',
    filterOptions: computed(() => {
        return Array.from(allTags.value).map(tag => ({ label: tag, value: tag }))
    }).value,
    filter(value, row) {
        return row.tags.includes(value as string)
    }
}
const timeColumn: DataTableBaseColumn<RowData> = {
    title: '创建时间',
    key: 'created_at',
    sortOrder: false,
    sorter(rowA, rowB) {
        console.log(timeColumn.sortOrder);
        // new Date()返回毫秒数, getTime()方法获取number
        return new Date(rowA.created_at).getTime() - new Date(rowB.created_at).getTime()
    }
}

// 枚举映射表
const statusOrder = {
    draft: 1,
    archived: 2,
    published: 3,
}
const StatusColumn: DataTableBaseColumn<RowData> = {
    title: 'Status',
    key: 'status',
    sortOrder: false,
    render(row: { status: any; }) {
        // h函数创建一个虚拟DOM节点
        return h(StatusTag, { status: row.status })
    },
    sorter(rowA, rowB) {
        console.log(StatusColumn.sortOrder); // -1
        // 下标访问对应属性值
        return statusOrder[rowA.status] - statusOrder[rowB.status]
    }
}
const ActionColumn: DataTableBaseColumn<RowData> =
{
    title: '操作',
    key: 'actions',
    render(row: { id: any; status: any; }) {
        return h(
            ArticleAction, { id: row.id, status: row.status, onEdit: handleEdit, onToggleStatus: handleToggleStatus, onDelete: handleDelete }
        )
    }
}

console.log(timeColumn.sortOrder)
console.log(StatusColumn.sortOrder)

const timeColumnReactive = reactive(timeColumn)
const StatusColumnReactive = reactive(StatusColumn)

// 受控的排序，表格列
const columns: DataTableBaseColumn<RowData>[] = [
    titleColumn,
    timeColumn,
    StatusColumn,
    ActionColumn,
]

// 页码
const pagination = {
    pageSize: 50
}
const columnsRef = ref<DataTableBaseColumn<RowData>[]>(columns)

// 页面加载时检查用户权限并加载文章
onMounted(() => {
    if (!userStore.isAdmin()) {
        router.push('/'); // 如果不是管理员，重定向到首页
        return;
    }
    loadArticles();
})

// 表格数据
const loadArticles = async () => {

    const res = await fetchArticles("admin");
    // axios库会把数据封装在res.data里
    data.value = res.data.articles.map((item: Article) => ({
        id: item.id,
        title: item.title,
        created_at: item.created_at,
        status: item.status,
        tags: item.tags,
    }));
    console.log(data.value);
}



// 新建文章
const handleAdd = () => {
    router.push('/admin/create');
}

// 编辑文章
const handleEdit = (id: Article["id"]) => {
    router.push(`/admin/edit/${id}`);
}

// 删除文章
const handleDelete = async (id: Article["id"]) => {
    dialog.warning({
        title: '确认删除?',
        content: '此操作将永久删除该文章，是否继续?',
        positiveText: '确定',
        negativeText: '取消',
        onPositiveClick: async () => {
            // 模拟删除接口
            const res = await deleteArticle(id)
            if (res && res.status == 204) {
                message.success("Delete succeeded");
            } else if (res && res.data.message == "删除失败") {
                message.error("Delete failed");
            } else {
                message.warning("operation failed");
            }
            // 删除后再次拉取数据
            await loadArticles()
        }
    })
}

// 转换文章状态
const handleToggleStatus = async (id: Article["id"], toggle: string) => {

    // console.log(userStore.token);


    await toggleStatus(id, toggle)
    const index = data.value.findIndex((a) => a.id == id)
    if (index !== -1 && data.value[index].status !== toggle) {
        // 索引访问类型（indexed access type） 的写法，读作“把 Article 这个类型里名叫 status 的属性的类型拿出来”。
        data.value[index].status = (toggle) as Article["status"];
        // console.log(data.value[index].status);
        articleStore.updateFolderContentSignal = true // 通知侧边导航栏更新目录
    }
}

// 处理排序
const handleSorterChange = (sorter: DataTableSortState) => {
    columnsRef.value.forEach((column: DataTableBaseColumn<RowData>) => {
        /** column.sortOrder !== undefined means it is uncontrolled */
        if (column.sortOrder === undefined)
            return
        if (!sorter) {
            column.sortOrder = false
            return
        }
        if (column.key === sorter.columnKey) {
            column.sortOrder = sorter.order
            // console.log(`${column.key}, ${sorter.columnKey}, ${column.sortOrder},${sorter.order}`);

        }

        else { column.sortOrder = false }
    })
}

// 清空排序
const clearSorter = async () => {
    // 表格排序
    // timeColumnReactive.sortOrder = false
    // StatusColumnReactive.sortOrder = false
    // // 标签筛选
    // titleColumn.filterOptionValue = null

    tableRef.value.filter(null)
    tableRef.value.sort(null)
    // await loadArticles()
}



// 非受控过滤数据表格, 初始化并更新tag
watchEffect(() => {
    const tagSet = new Set<string>()
    data.value.forEach(row => {
        tagSet.add(row.tags)
    })
    titleColumn.filterOptions = Array.from(tagSet).map(tag => ({
        // 箭头函数如果直接返回一个 对象字面量，必须用 圆括号 () 包裹起来。否则 JavaScript 会误以为你写的是函数体，而不是一个对象。
        label: tag,
        value: tag
    }))
})

</script>
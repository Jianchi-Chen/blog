<template>
    <n-button type="primary" @click="handleAdd">新建文章</n-button>
    <n-data-table :columns="columns" :data="data" :pagination="pagination" :bordered="true"></n-data-table>
</template>

<script setup lang="ts">
import { NDataTable, NButton, useDialog, useMessage } from 'naive-ui';
import type { DataTableColumns } from 'naive-ui'
import { deleteArticle, fetchArticleById, fetchArticles, toggleStatus } from '@/api/article';
import { useUserStore } from '@/stores/user';
import { h, onMounted, ref, render } from 'vue';
import { useRouter } from 'vue-router';
import axios from '@/api/client';
import StatusTag from '@/components/StatusTag.vue';
import ArticleAction from '@/components/ArticleAction.vue';
import type { Article } from '@/types/article';
import type { EnumType } from 'typescript';


const loading = ref(true);

const router = useRouter();
const userStore = useUserStore();
const dialog = useDialog();
const message = useMessage();

// 表格列
const columns = ref([
    {
        title: '标题',
        key: 'title',
    },
    {
        title: '创建时间',
        key: 'created_at',
    },
    {
        title: '状态',
        key: 'status',
        render(row: { status: any; }) {
            // h函数创建一个虚拟DOM节点
            return h(StatusTag, { status: row.status })
        }
    },
    {
        title: '操作',
        key: 'actions',
        render(row: { id: any; status: any; }) {
            return h(
                ArticleAction, { id: row.id, status: row.status, onEdit: handleEdit, onToggleStatus: handleToggleStatus, onDelete: handleDelete }
            )
        }
    },
]);



// 页码
const pagination = {
    pageSize: 50
}

// 页面加载时检查用户权限并加载文章
onMounted(() => {
    if (!userStore.isAdmin()) {
        router.push('/'); // 如果不是管理员，重定向到首页
        return;
    }
    loadArticles();
})

// 表格数据
const data = ref<Article[]>([]);
const loadArticles = async () => {

    const res = await fetchArticles("admin");
    // axios库会把数据封装在res.data里
    data.value = res.data.map((item: Article) => ({
        id: item.id,
        title: item.title,
        created_at: item.created_at,
        status: item.status,
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
            if (res && res.data.message == "删除成功") {
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
    await toggleStatus(id, toggle)
    const index = data.value.findIndex((a) => a.id == id)
    if (index !== -1 && data.value[index].status !== toggle) {
        // 索引访问类型（indexed access type） 的写法，读作“把 Article 这个类型里名叫 status 的属性的类型拿出来”。
        data.value[index].status = (toggle) as Article["status"];
        console.log(data.value[index].status);
    }
}


</script>
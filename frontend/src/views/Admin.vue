<template>
    <n-button type="primary" @click="handleAdd">新建文章</n-button>
    <n-data-table :columns="columns" :data="data" :pagination="pagination" :bordered="true"></n-data-table>
</template>

<script setup lang="ts">
import { NDataTable, NButton } from 'naive-ui';
import type { DataTableColumns } from 'naive-ui'
import { fetchArticleById, fetchArticles } from '@/api/article';
import { useUserStore } from '@/stores/user';
import { h, onMounted, ref, render } from 'vue';
import { useRouter } from 'vue-router';
import axios from 'axios';

const loading = ref(true);
const articles = ref<any[]>([]);
const router = useRouter();
const userStore = useUserStore();

const columns = [
    {
        title: 'ID',
        key: 'id',
    },
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
        render(row: { status: string; }) {
            return row.status === 'published' ? '已发布' : '草稿'
        }
    },
    {
        title: '操作',
        key: 'actions',
        render(row: { id: string | number; }) {
            return h(
                'div',
                [
                    // 手动创建虚拟 DOM 元素
                    h(
                        NButton,
                        {
                            strong: true,
                            tertiary: true,
                            size: 'small',
                            onClick: () => handleEdit(row.id)
                        },
                        { default: () => '编辑' }
                    ),
                    ' | ',
                    h(
                        NButton,
                        {
                            strong: true,
                            tertiary: true,
                            size: 'small',
                            onClick: () => handleDelete(row.id)
                        },
                        { default: () => '删除' }
                    ),
                ]
            )
        }
    },
]

// 页码
const pagination = {
    pageSize: 5
}

// 页面加载时检查用户权限并加载文章
onMounted(() => {
    if (!userStore.isAdmin()) {
        router.push('/'); // 如果不是管理员，重定向到首页
        return;
    }
    loadArticles();
})

// 使用到的数据结构
interface articleDataModel {
    id: number,
    title: string,
    created_at: string,
    status: string,
}
const data = ref<articleDataModel[]>([]);
const loadArticles = async () => {

    const res = await fetchArticles();
    // axios库会把数据封装在res.data里
    data.value = res.data.map((item: articleDataModel) => ({
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
const handleEdit = (id: number | string) => {
    router.push(`/admin/edit/${id}`);
}

// 删除文章
const handleDelete = async (id: number | string) => {
    await axios.post('/articleDelete', id)
    await fetchArticles();

}


</script>
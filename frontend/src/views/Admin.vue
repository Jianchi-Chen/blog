<template>
    <div>
        <h1>后台管理</h1>
        <div><button @click="handleAdd">新建文章</button></div>

        <div v-if="loading">加载中...</div>

        <div v-else>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>标题</th>
                        <th>创建时间</th>
                        <th>操作</th>
                    </tr>
                </thead>

                <tbody>
                    <tr v-for="article in articles" :key="article.id">
                        <td>{{ article.id }}</td>
                        <td>{{ article.title }}</td>
                        <td>{{ article.created_at }}</td>
                        <td>
                            <button @click="handleEdit(article.id)">编辑</button>
                            <button @click="handleDelete(article.id)">删除</button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>

<script setup lang="ts">
import { fetchArticleById, fetchArticles } from '@/api/article';
import { useUserStore } from '@/stores/user';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const loading = ref(true);
const articles = ref<any[]>([]);
const router = useRouter();
const userStore = useUserStore();

const loadArticles = async () => {
    loading.value = true;
    try {
        const res = await fetchArticles();
        articles.value = res.data;
    } catch (err) {
        console.error("加载文章失败", err);
    } finally {
        loading.value = false;
    }
}

// 页面加载时检查用户权限并加载文章
// TODO: 此处可能存在问题, 关联文件: frontend/src/stores/user.ts
onMounted(() => {
    if (userStore.isAdmin()) {
        router.push('/'); // 如果不是管理员，重定向到首页
        return;
    }
    loadArticles();
})

// 新建文章
const handleAdd = () => {
    router.push('/admin/create');
}

// 编辑文章
const handleEdit = (id: number | string) => {
    router.push(`/admin/edit/${id}`);
}

const handleDelete = (id: number | string) => {
    // 这里可以调用删除API，暂时模拟删除
    articles.value = articles.value.filter(item => item.id !== id);
}

</script>
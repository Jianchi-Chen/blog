<template>
    <n-flex vertical :size="12">
        <n-flex>
            <n-button type="primary" @click="handleAdd">
                {{ newButton }}
            </n-button>
            <n-button @click="clearSorter"> Clear Sorter </n-button>
        </n-flex>

        <!-- 标签页 -->
        <n-card>
            <n-tabs @update:value="changeNewButton">
                <n-tab-pane name="articleAdmin" tab="文章管理">
                    <n-data-table
                        :columns="ArticleColumnsRef"
                        :data="data"
                        :pagination="pagination"
                        :bordered="true"
                        ref="tableRef"
                        @update:sorter="handleSorterChange"
                    ></n-data-table>
                </n-tab-pane>

                <n-tab-pane name="userAdmin" tab="用户管理">
                    <n-data-table
                        :columns="UserColumnsRef"
                        :data="Userdata"
                        :pagination="pagination"
                        :bordered="true"
                        ref="UsersTableRef"
                        @update:sorter="UsersSorterChange"
                    ></n-data-table>
                </n-tab-pane>
            </n-tabs>
        </n-card>

        <NewUserDialog
            v-model:show="showNewUserDialog"
            @success="handlerUserSuccess"
        />

        <EditUserDialog
            v-model:show="showEditUserDialog"
            v-model:userdata="SelectedUserdata"
            @success="handlerUserSuccess"
        />
    </n-flex>
</template>

<script setup lang="ts">
import { NDataTable, NButton, useDialog, useMessage, NTag } from "naive-ui";
import type { DataTableBaseColumn, DataTableSortState } from "naive-ui";
import { deleteArticle, fetchArticles, toggleStatus } from "@/api/article";
import { useUserStore } from "@/stores/user";
import { computed, h, onMounted, ref, watchEffect } from "vue";
import { useRouter } from "vue-router";
import StatusTag from "@/components/StatusTag.vue";
import ArticleAction from "@/components/ArticleAction.vue";
import type { Article } from "@/types/article";
import { useArticleStore } from "@/stores/article";
import type { User } from "@/types/user";
import { fetchUsers, deleteUser as deleteUserApi } from "@/api/account";
import { NewLineKind } from "typescript";
import NewUserDialog from "@/components/NewUserDialog.vue";
import { useAppStore } from "@/stores/app";

const loading = ref(true);
const router = useRouter();
const userStore = useUserStore();
const dialog = useDialog();
const message = useMessage();
const data = ref<Article[]>([]);
const tableRef = ref();
const UsersTableRef = ref();
const articleStore = useArticleStore();
const Userdata = ref<User[]>([]);
const newButton = ref("New Article");
const showNewUserDialog = ref(false);
const showEditUserDialog = ref(false);
const SelectedUserdata = ref<User | null>(null);
const appstore = useAppStore();

interface RowData {
    id: string;
    key: string;
    title: string;
    created_at: string;
    status: Article["status"];
    tags: string[];
}

interface UserRowData {
    id: string;
    key: string;
    username: string;
    identity: string;
    created_at: string;
}

// 收集所有tag，返回string[]
const allTags = computed(() => {
    const set = new Set<string>();
    data.value.forEach((i) => {
        set.add(i.tags);
    });
    return Array.from(set);
});
// 标签筛选
const titleColumn: DataTableBaseColumn<RowData> = {
    title: "标题",
    key: "title",
    filterOptions: computed(() => {
        return Array.from(allTags.value).map((tag) => ({
            label: tag,
            value: tag,
        }));
    }).value,
    filter(value, row) {
        return row.tags.includes(value as string);
    },
};
const timeColumn: DataTableBaseColumn<RowData> = {
    title: "创建时间",
    key: "created_at",
    sortOrder: false,
    sorter(rowA, rowB) {
        console.log(timeColumn.sortOrder);
        // new Date()返回毫秒数, getTime()方法获取number
        return (
            new Date(rowA.created_at).getTime() -
            new Date(rowB.created_at).getTime()
        );
    },
};

// 枚举映射表
const statusOrder = {
    draft: 1,
    archived: 2,
    published: 3,
};
const StatusColumn: DataTableBaseColumn<RowData> = {
    title: "Status",
    key: "status",
    sortOrder: false,
    render(row: { status: any }) {
        // h函数创建一个虚拟DOM节点
        return h(StatusTag, { status: row.status });
    },
    sorter(rowA, rowB) {
        console.log(StatusColumn.sortOrder); // -1
        // 下标访问对应属性值
        return statusOrder[rowA.status] - statusOrder[rowB.status];
    },
};
const ActionColumn: DataTableBaseColumn<RowData> = {
    title: "操作",
    key: "actions",
    render(row: { id: any; status: any }) {
        return h(ArticleAction, {
            id: row.id,
            status: row.status,
            onEdit: handleEdit,
            onToggleStatus: handleToggleStatus,
            onDelete: handleDelete,
        });
    },
};

console.log(timeColumn.sortOrder);
console.log(StatusColumn.sortOrder);

// 受控的排序，表格列
const columns: DataTableBaseColumn<RowData>[] = [
    titleColumn,
    timeColumn,
    StatusColumn,
    ActionColumn,
];

// 页码
const pagination = {
    pageSize: 50,
};
const ArticleColumnsRef = ref<DataTableBaseColumn<RowData>[]>(columns);

// 用户管理表格列
const UserColumnsRef = ref<DataTableBaseColumn<UserRowData>[]>([
    {
        title: "用户名",
        key: "username",
    },
    {
        title: "身份",
        key: "identity",
        sorter(rowA, rowB) {
            const identityOrder: Record<string, number> = {
                admin: 3,
                user: 2,
                visitor: 1,
            };
            return identityOrder[rowA.identity] - identityOrder[rowB.identity];
        },
        render(row: { identity: string }) {
            return h("div", {}, [
                h(
                    NTag,
                    {
                        type:
                            row.identity === "admin"
                                ? "error"
                                : row.identity === "user"
                                ? "success"
                                : "warning",
                    },
                    {
                        default: () => row.identity,
                    }
                ),
            ]);
        },
    },
    {
        title: "操作",
        key: "actions",
        render(row: { id: any }) {
            return h("div", {}, [
                h(
                    NButton,
                    {
                        type: "primary",
                        size: "small",
                        onClick: () => {
                            SelectedUserdata.value = Userdata.value.find(
                                (user) => user.id === row.id
                            ) as User;

                            showEditUserDialog.value = true;
                        },
                    },
                    { default: () => "编辑" }
                ),
                h(
                    NButton,
                    {
                        type: "error",
                        size: "small",
                        style: { "margin-left": "8px" },
                        onClick: () => {
                            deleteUser(row.id);
                        },
                    },
                    { default: () => "删除" }
                ),
            ]);
        },
    },
]);

// 页面加载时检查用户权限并加载文章
onMounted(() => {
    if (!userStore.isAdmin()) {
        router.push("/"); // 如果不是管理员，重定向到首页
        return;
    }
    loadArticlesAndUsers();
});

// 表格数据
const loadArticlesAndUsers = async () => {
    const res = await fetchArticles("admin");
    // axios库会把数据封装在res.data里
    data.value = res.data.articles.map((item: Article) => ({
        id: item.id,
        title: item.title,
        created_at: item.created_at,
        status: item.status,
        tags: item.tags,
    }));

    const resUsers = await fetchUsers(20);
    Userdata.value = resUsers.data.users.map((item: User) => ({
        id: item.id,
        username: item.username,
        identity: item.identity,
    }));

    //初始化用户表格排序
    UserColumnsRef.value.forEach((column: DataTableBaseColumn<UserRowData>) => {
        if (column.key === "identity") {
            column.sortOrder = "descend";
        } else {
            column.sortOrder = false;
        }
    });
};

// 新建文章/用户
const handleAdd = () => {
    if (newButton.value === "New User") {
        // 弹出一个窗口，其中显示创建的对话框
        showNewUserDialog.value = true;
        return;
    }
    router.push("/admin/createArticle");
};

// 编辑文章
const handleEdit = (id: Article["id"]) => {
    router.push(`/admin/edit/${id}`);
};

// 删除文章
const handleDelete = async (id: Article["id"]) => {
    dialog.warning({
        title: "确认删除?",
        content: "此操作将永久删除该文章，是否继续?",
        positiveText: "确定",
        negativeText: "取消",
        onPositiveClick: async () => {
            // 模拟删除接口
            const res = await deleteArticle(id);
            if (
                appstore.isTauri
                    ? (res.data as any).message === "done"
                    : (res as any).status === 204
            ) {
                message.success("Delete succeeded");
            } else if (res && res.data.message == "删除失败") {
                message.error("Delete failed");
            } else {
                message.warning("operation failed");
            }
            // 删除后再次拉取数据
            await loadArticlesAndUsers();
        },
    });
};

// 转换文章状态
const handleToggleStatus = async (id: Article["id"], toggle: string) => {
    // console.log(userStore.token);

    await toggleStatus(id, toggle);
    const index = data.value.findIndex((a) => a.id == id);
    if (index !== -1 && data.value[index].status !== toggle) {
        // 索引访问类型（indexed access type） 的写法，读作“把 Article 这个类型里名叫 status 的属性的类型拿出来”。
        data.value[index].status = toggle as Article["status"];
        articleStore.updateFolderContentSignal = true; // 通知侧边导航栏更新目录
    }
};

// 文章管理处理排序
const handleSorterChange = (sorter: DataTableSortState) => {
    ArticleColumnsRef.value.forEach((column: DataTableBaseColumn<RowData>) => {
        /** column.sortOrder !== undefined means it is uncontrolled */
        if (column.sortOrder === undefined) return;
        if (!sorter) {
            column.sortOrder = false;
            return;
        }
        if (column.key === sorter.columnKey) {
            column.sortOrder = sorter.order;
        } else {
            column.sortOrder = false;
        }
    });
};

// 用户管理处理排序
const UsersSorterChange = (sorter: DataTableSortState) => {
    UserColumnsRef.value.forEach((column: DataTableBaseColumn<UserRowData>) => {
        if (column.sortOrder === undefined) return;
        if (!sorter) {
            column.sortOrder = false;
            return;
        }
        if (column.key === sorter.columnKey) {
            column.sortOrder = sorter.order;
        } else {
            column.sortOrder = false;
        }
    });
};

// 清空排序(默认清空所有数据表格的排序)
const clearSorter = async () => {
    tableRef.value.filter(null);
    tableRef.value.sort(null);
    // UsersTableRef.value.filter(null);
    // UsersTableRef.value.sort(null);
};

// 删除用户
const deleteUser = async (id: User["id"]) => {
    dialog.warning({
        title: "确认删除?",
        content: "此操作将永久删除该用户，是否继续?",
        positiveText: "确定",
        negativeText: "取消",
        onPositiveClick: async () => {
            // guard against undefined/null id so the API always receives string|number
            if (id == null) {
                message.error("无效的用户ID, 无法删除");
                return;
            }
            try {
                const res = await deleteUserApi(id);
                if (
                    appstore.isTauri
                        ? (res.data as any).message === "done"
                        : (res as any).status === 204
                ) {
                    message.success("用户删除成功");
                    // 删除后更新用户表格数据
                    Userdata.value = Userdata.value.filter(
                        (user) => user.id !== id
                    );
                } else {
                    message.error("用户删除失败");
                }
            } catch (error) {
                message.error("请求出错，无法删除用户");
            }
        },
    });
};

const changeNewButton = (value: string) => {
    console.log("当前选中的标签是:", value);
    newButton.value === "New Article"
        ? (newButton.value = "New User")
        : (newButton.value = "New Article");
};

// 新建或是更新用户信息成功后的回调
const handlerUserSuccess = () => {
    // 重新加载用户数据
    showNewUserDialog.value = false;
    showEditUserDialog.value = false;
    loadArticlesAndUsers();
};

// 非受控过滤数据表格, 初始化并更新tag
watchEffect(() => {
    const tagSet = new Set<string>();
    data.value.forEach((row) => {
        tagSet.add(row.tags);
    });
    titleColumn.filterOptions = Array.from(tagSet).map((tag) => ({
        // 箭头函数如果直接返回一个 对象字面量，必须用 圆括号 () 包裹起来。否则 JavaScript 会误以为你写的是函数体，而不是一个对象。
        label: tag,
        value: tag,
    }));
});
</script>

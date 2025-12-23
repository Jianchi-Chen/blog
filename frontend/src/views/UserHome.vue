<template>
    <div class="user-home">
        <n-grid :x-gap="24" :cols="3" :cols-sm="1">
            <!-- 侧栏：头像 + 签名 -->
            <n-grid-item :span="1">
                <n-card class="profile-card" size="large" bordered>
                    <div class="profile-top">
                        <div class="avatar-wrap">
                            <n-avatar
                                :size="96"
                                :src="avatar || undefined"
                                :text="initials"
                                @click="showAvatar"
                                :lazy="true"
                                object-fit="cover"
                                style="cursor: pointer"
                            />
                            <div class="avatar-actions">
                                <n-button
                                    size="tiny"
                                    secondary
                                    @click="selectAvatar"
                                    >上传头像</n-button
                                >
                                <n-button
                                    size="tiny"
                                    tertiary
                                    @click="removeAvatar"
                                    >移除</n-button
                                >
                            </div>
                        </div>
                        <div class="profile-info">
                            <div class="username">{{ user.name }}</div>
                            <div class="meta">ID: {{ user.id }}</div>
                        </div>
                    </div>

                    <n-divider />

                    <!-- 签名 -->
                    <div class="signature-section">
                        <div class="section-header">
                            <div class="title">个性签名</div>
                            <div>
                                <n-button
                                    size="tiny"
                                    @click="
                                        editingSignature = !editingSignature
                                    "
                                >
                                    {{ editingSignature ? "取消" : "编辑" }}
                                </n-button>
                            </div>
                        </div>

                        <div v-if="editingSignature" class="signature-edit">
                            <n-input
                                type="textarea"
                                v-model:value="draftSignature"
                                :rows="3"
                                placeholder="写下你的个性签名..."
                                maxlength="120"
                                show-count
                            />
                            <div class="actions">
                                <n-button
                                    size="small"
                                    @click="saveSignature"
                                    type="primary"
                                    >保存</n-button
                                >
                                <n-button size="small" @click="cancelSignature"
                                    >取消</n-button
                                >
                            </div>
                        </div>

                        <div v-else class="signature-display">
                            <div v-if="signature" class="signature-text">
                                "{{ signature }}"
                            </div>
                            <n-empty
                                v-else
                                description="还没有签名，去编辑一个吧"
                            />
                        </div>
                    </div>
                </n-card>
            </n-grid-item>

            <!-- 主要：收藏文章列表 -->
            <n-grid-item :span="2">
                <n-card
                    class="fav-card"
                    size="large"
                    title="收藏的文章"
                    bordered
                >
                    <div class="list-controls">
                        <n-input
                            size="small"
                            clearable
                            v-model:value="filter"
                            placeholder="搜索文章标题或标签"
                            style="max-width: 320px"
                        />
                        <n-button size="small" @click="refreshList"
                            >刷新</n-button
                        >
                    </div>

                    <n-divider />

                    <div v-if="loading" class="loading-wrap">
                        <n-spin size="large" />
                    </div>

                    <div v-else>
                        <n-list bordered>
                            <n-list-item
                                v-for="item in filteredFavorites"
                                :key="item.id"
                                class="fav-item"
                            >
                                <div class="item-left">
                                    <div class="item-title">
                                        {{ item.title }}
                                    </div>
                                    <div class="item-desc">
                                        {{ item.excerpt }}
                                    </div>
                                    <div class="item-meta">
                                        <n-tag
                                            v-for="tag in item.tags"
                                            :key="tag"
                                            small
                                            >{{ tag }}</n-tag
                                        >
                                        <span class="spacer"></span>
                                        <span class="date">{{
                                            formatDate(item.updatedAt)
                                        }}</span>
                                    </div>
                                </div>

                                <div class="item-actions">
                                    <n-button
                                        size="tiny"
                                        @click="openArticle(item)"
                                        >查看</n-button
                                    >
                                    <n-button
                                        size="tiny"
                                        secondary
                                        @click="unfavorite(item.id)"
                                        >取消收藏</n-button
                                    >
                                </div>
                            </n-list-item>
                        </n-list>

                        <div
                            v-if="filteredFavorites.length === 0"
                            class="empty-wrap"
                        >
                            <n-empty description="没有找到符合条件的文章" />
                        </div>
                    </div>
                </n-card>
            </n-grid-item>
        </n-grid>

        <!-- 头像大图查看模态框 -->
        <n-modal
            v-model:show="showAvatarModal"
            preset="card"
            title="用户头像"
            :style="{ width: '80%', maxWidth: '800px' }"
            :bordered="false"
        >
            <div class="avatar-preview">
                <img :src="avatar" alt="用户头像" />
            </div>
        </n-modal>
    </div>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import { uploadAvatar } from "@/api/account";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { appDataDir, join } from "@tauri-apps/api/path";
import { readFile } from "@tauri-apps/plugin-fs";
import {
    NGrid,
    NGridItem,
    NCard,
    NAvatar,
    NButton,
    NInput,
    NDivider,
    NEmpty,
    NList,
    NListItem,
    NTag,
    NSpin,
    useMessage,
    NModal,
} from "naive-ui";
import { ref, reactive, computed, onMounted } from "vue";

interface FavoriteArticle {
    id: string;
    title: string;
    excerpt: string; // 同article.summary
    tags: string[];
    updatedAt: number;
    link?: string;
}

const message = useMessage();
const userstore = useUserStore();
const showAvatarModal = ref(false);
// 用户基础信息（示例）
const user = reactive({
    id: userstore.id || "未发现ID",
    name: userstore.username || "未发现用户名",
});

// 头像（base64 URL）
const avatar = ref(localStorage.getItem("user_avatar") || "");

const initials = computed(() => {
    const parts = user.name.split(" ");
    if (parts.length >= 2) {
        return (parts[0][0] + parts[1][0]).toUpperCase();
    }
    return user.name.slice(0, 2).toUpperCase();
});

// 个性签名
const signature = ref(localStorage.getItem("user_signature") || "");
const draftSignature = ref(signature.value);
const editingSignature = ref(false);

// 收藏文章（示例数据，真实项目请替换为接口请求）
const loading = ref(true);
const favorites = ref<FavoriteArticle[]>([]);

const sampleData: FavoriteArticle[] = [
    {
        id: "a1",
        title: "用 Vue 构建现代博客的最佳实践",
        excerpt:
            "分享如何使用 Vue 3 + Naive UI 快速构建一个简洁且可维护的博客前端。",
        tags: ["Vue", "Naive UI", "前端"],
        updatedAt: Date.now() - 1000 * 60 * 60 * 24 * 2,
        link: "https://example.com/article/a1",
    },
    {
        id: "a2",
        title: "前端性能优化：实战指南",
        excerpt: "从资源加载到渲染优化，带你一步步提升页面表现。",
        tags: ["性能", "优化"],
        updatedAt: Date.now() - 1000 * 60 * 60 * 24 * 10,
        link: "https://example.com/article/a2",
    },
];

const loadFavorites = () => {
    loading.value = true;
    // 模拟接口延时
    setTimeout(() => {
        // 如果本地有缓存则加载
        const cached = localStorage.getItem("user_favorites");
        if (cached) {
            favorites.value = JSON.parse(cached);
        } else {
            favorites.value = sampleData;
            localStorage.setItem("user_favorites", JSON.stringify(sampleData));
        }
        loading.value = false;
    }, 600);
};

// 搜索过滤
const filter = ref("");
const filteredFavorites = computed(() => {
    if (!filter.value) return favorites.value;
    const q = filter.value.toLowerCase();
    return favorites.value.filter(
        (i) =>
            i.title.toLowerCase().includes(q) ||
            i.tags.some((t) => t.toLowerCase().includes(q)) ||
            (i.excerpt && i.excerpt.toLowerCase().includes(q))
    );
});

const refreshList = () => {
    loadFavorites();
};

const openArticle = (item: FavoriteArticle) => {
    // 真实项目可以使用 router.push(...)
    window.open(item.link || "#", "_blank");
};

const unfavorite = (id: string) => {
    favorites.value = favorites.value.filter((i) => i.id !== id);
    localStorage.setItem("user_favorites", JSON.stringify(favorites.value));
};

// 头像选择和上传：使用 Tauri 文件选择器
const selectAvatar = async () => {
    try {
        // 打开文件选择对话框
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: "图片",
                    extensions: ["png", "jpg", "jpeg"],
                },
            ],
        });

        if (!selected) {
            return; // 用户取消选择
        }

        // selected 是文件路径字符串
        const filePath = selected as string;

        // 调用后端保存头像
        const result = await uploadAvatar(filePath);
        console.log("result:", result);

        // 构建完整的文件路径
        const dataDir = await appDataDir();
        const fullPath = await join(dataDir, result.path);
        console.log("fullPath:", fullPath);

        // 读取文件内容并转换为 base64
        const fileBytes = await readFile(fullPath);
        const base64 = btoa(
            new Uint8Array(fileBytes).reduce(
                (data, byte) => data + String.fromCharCode(byte),
                ""
            )
        );

        // 根据文件扩展名确定 MIME 类型
        const ext = result.path.split(".").pop()?.toLowerCase();
        const mimeType =
            ext === "png"
                ? "image/png"
                : ext === "jpg" || ext === "jpeg"
                ? "image/jpeg"
                : "image/jpeg";

        const base64Url = `data:${mimeType};base64,${base64}`;

        // 保存到 localStorage 和更新显示
        avatar.value = base64Url;
        localStorage.setItem("user_avatar", base64Url);

        message.success("头像上传成功！");
    } catch (error: any) {
        console.error("Upload avatar error:", error, {
            duration: 50000,
            closable: true,
        });
        message.error(error || "头像上传失败");
    }
};

const removeAvatar = () => {
    avatar.value = "";
    localStorage.removeItem("user_avatar");
};

// 签名编辑
const saveSignature = () => {
    signature.value = draftSignature.value.trim();
    localStorage.setItem("user_signature", signature.value);
    editingSignature.value = false;
};

const cancelSignature = () => {
    draftSignature.value = signature.value;
    editingSignature.value = false;
};

const formatDate = (ts: number) => {
    const d = new Date(ts);
    return (
        d.toLocaleDateString() +
        " " +
        d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
    );
};

// 查看头像大图
const showAvatar = () => {
    console.log("showAvator triggle out");

    if (avatar.value) {
        showAvatarModal.value = true;
        const imgWindow = window.open("", "_blank", "noopener,noreferrer");
        if (imgWindow) {
            const doc = imgWindow.document;
            const img = doc.createElement("img");
            img.src = avatar.value;
            img.style.maxWidth = "100%";
            img.style.height = "auto";
            if (doc.body) {
                doc.body.style.margin = "0";
                doc.body.appendChild(img);
            }
            img.style.maxWidth = "100%";
            img.style.height = "auto";
            doc.body.appendChild(img);
        }
    } else {
        message.info("当前没有设置头像");
    }
};

onMounted(() => {
    loadFavorites();
});
</script>

<style scoped>
.user-home {
    padding: 20px;
}

.profile-card {
    height: 100%;
}

.profile-top {
    display: flex;
    gap: 16px;
    align-items: center;
}

.avatar-wrap {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
}

.avatar-actions {
    display: flex;
    gap: 8px;
}

.profile-info .username {
    font-weight: 600;
    font-size: 18px;
}

.profile-info .meta {
    color: var(--n-typography-3);
    font-size: 12px;
    margin-top: 6px;
}

.signature-section .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.signature-edit .actions {
    margin-top: 8px;
    display: flex;
    gap: 8px;
}

.signature-display .signature-text {
    font-style: italic;
    color: var(--n-typography-2);
    padding: 8px 0;
}

.fav-card .list-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    margin-bottom: 8px;
}

.loading-wrap {
    display: flex;
    justify-content: center;
    padding: 40px 0;
}

.fav-item {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 12px 8px;
}

.item-left {
    max-width: 72%;
}

.item-title {
    font-weight: 600;
    margin-bottom: 6px;
}

.item-desc {
    color: var(--n-typography-3);
    font-size: 13px;
    margin-bottom: 8px;
}

.item-meta {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
}

.item-actions {
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: flex-end;
}

.spacer {
    width: 10px;
}

.empty-wrap {
    padding: 24px 0;
    display: flex;
    justify-content: center;
}

.avatar-preview {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 40px 0;
}

.avatar-preview img {
    max-width: 100%;
    height: auto;
    border-radius: 8px;
}
</style>

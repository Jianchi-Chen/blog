# Copilot Instructions - myBlog

## 项目概述

这是一个基于 **Vue 3 + Tauri 2** 的博客应用，支持双模式运行：

-   **Tauri 桌面应用**：使用 Rust 后端 + SQLite 数据库
-   **纯 Web 应用**：使用 Mock Service Worker (MSW) 模拟 API

## 核心架构

### 双环境 API 策略

**关键文件**: [src/api/client.ts](../src/api/client.ts)

所有 API 请求统一通过 `client.ts` 处理，自动适配运行环境：

-   **Web 环境**：使用 Axios + Bearer token + MSW mock
-   **Tauri 环境**：请求会被前端拦截并通过 Rust 命令 `http_request` 转发
    -   Token 通过 `__token` 字段传递（非标准 HTTP header）
    -   参见 [src-tauri/src/commands/http.rs](../src-tauri/src/commands/http.rs)

```typescript
// 所有 API 调用使用统一客户端
import client from "@/api/client";
const articles = await client.get("/articles", {
    params: { identity: "public" },
});
```

### Tauri 后端架构

**核心入口**: [src-tauri/src/lib.rs](../src-tauri/src/lib.rs)

-   **数据库**：SQLite + SQLx，迁移文件在 `src-tauri/migrations/`
-   **认证**：JWT + Argon2 密码哈希，详见 [src-tauri/AUTH_GUIDE.md](../src-tauri/AUTH_GUIDE.md)
-   **命令模块化**：`src-tauri/src/commands/` 目录按功能拆分
-   **状态管理**：Config 和 SqlitePool 通过 `app.manage()` 全局注入

### 前端状态管理

**Pinia stores** - [src/stores/](../src/stores/):

-   `app.ts`: 检测 Tauri 环境 (`isTauri()`)
-   `user.ts`: JWT token + localStorage 持久化，**所有身份验证依赖此 store**
-   `article.ts`: 文章列表缓存
-   `search.ts`: 搜索历史

## 关键开发模式

### 本地开发

```bash
# Web 开发 (MSW mock)
npm run dev

# Tauri 开发 (完整 Rust 后端)
npm run tauri:dev
```

### Mock 数据开发

**Mock handlers**: [src/mocks/handlers.ts](../src/mocks/handlers.ts)

使用 MSW 2.x 语法，在浏览器环境下自动拦截 `/api/*` 和 `/articles` 等请求。

-   启动：[src/mocks/browser.ts](../src/mocks/browser.ts) 在 `main.ts` 中初始化
-   添加新 endpoint 时同时更新 handlers 和 Tauri commands

### Markdown 编辑器

使用 **Vditor** 库：

-   编辑：[src/components/MarkdownEditor.vue](../src/components/MarkdownEditor.vue)
-   预览：[src/components/MdPreview.vue](../src/components/MdPreview.vue)
-   每个编辑器实例需要唯一 ID（使用 `uuidv7()`）以启用缓存

## 重要约定

### 路由守卫

[../src/router/index.ts](../src/router/index.ts) 实现全局前置守卫：

-   `meta: { requiresAdmin: true }` 路由会检查 `user.identity === 'admin'`
-   未授权自动重定向到 `/login`

### 类型验证

使用 **Zod** 进行运行时类型校验（如 [../src/types/article.ts](../src/types/article.ts)）：

```typescript
export const ArticleSchema = z.object({ ... })
export type Article = z.infer<typeof ArticleSchema>
```

### UI 组件库

-   **Naive UI** - 自动导入配置在 [vite.config.ts](../vite.config.ts)
-   **Tailwind CSS 4.x** + `@tailwindcss/typography` 用于 Markdown 样式
-   **@vicons** 图标库（ionicons5/material/carbon）

### 构建配置

[../vite.config.ts](../vite.config.ts) 关键点：

-   `base` 路径根据环境切换：Tauri 用 `/`，Web 部署用 `/myBlog/`
-   `unplugin-vue-components` 自动生成 `components.d.ts`
-   代理配置：`/api` 转发到本地服务器（仅开发模式）

## 数据库操作

### 运行迁移

迁移在应用启动时自动执行（见 [../src-tauri/src/lib.rs](../src-tauri/src/lib.rs) 第 52 行）

```rust
run_migrations(&pool).await?;
run_seeds(&pool).await?; // 仅开发环境
```

### 添加新迁移

1. 在 `src-tauri/migrations/` 创建 `XXXX_description.sql`
2. 使用 SQLite 语法，支持向上迁移（无向下迁移）
3. 重启应用自动应用

## 常见陷阱

1. **Token 传递**：Tauri 环境下不要依赖 HTTP headers，使用 `__token` 字段
2. **路径问题**：确保使用 `@/` 别名而非相对路径
3. **Vditor 初始化**：必须在 `onMounted` 后创建实例，避免 SSR 问题
4. **环境检测**：使用 `useAppStore().isTauri` 而非直接调用 `isTauri()`
5. **数据库路径**：Tauri 配置中使用 `Config::get_database_path()` 获取正确路径

## 文件结构关键点

-   `src/api/` - API 调用封装，所有请求走 `client.ts`
-   `src/views/` - 页面组件（Admin\* 需要管理员权限）
-   `src-tauri/src/repositories/` - 数据库访问层（Repository 模式）
-   `src-tauri/src/models/` - Rust 数据模型（与前端 `src/types/` 对应）

## 调试技巧

-   **前端日志**：浏览器控制台 + Vue DevTools
-   **Tauri 日志**：查看终端输出或使用 `tauri-plugin-log`
-   **数据库检查**：应用数据目录下的 `app.db` 文件（SQLite）
-   **MSW 调试**：打开网络面板，mock 请求会标记 "from service worker"

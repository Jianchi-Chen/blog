# Copilot Instructions - myBlog

> **⚠️ 项目声明**：这是一个用于学习和练手的项目，代码并不严谨，随缘维护，仅供参考学习。项目不会部署到生产环境，未来可能会快速迭代修改。**不建议直接实际使用**，但欢迎交流探讨！

## 项目概述

这是一个练习博客项目，采用**三层架构**：

1. **backend/** - 独立 Rust API 服务器 (Axum + SQLite)
2. **frontend/** - Vue 3 Web 应用 (支持 MSW mock 数据)
3. **frontend/src-tauri/** - Tauri 桌面应用 (内嵌 Rust 后端 + SQLite)

**关键决策**：前端支持双模式运行 - Web 开发时用 MSW mock，Tauri 桌面应用直接调用内嵌的 Rust 命令。

## 三层架构细节

### 1. Backend - 独立 API 服务器

**入口**: [backend/src/main.rs](../backend/src/main.rs)

-   **框架**: Axum + Tower HTTP + CORS
-   **数据库**: SQLite + SQLx (无 macros，使用纯 SQL)
-   **认证**: JWT + Argon2 密码哈希
-   **迁移**: 启动时自动运行 `migrations/` 下的 SQL 文件

**启动命令**:

```bash
cd backend
cargo run  # 开发环境，监听 127.0.0.1:3000 (默认)
cargo build --release  # 生产构建
```

**路由结构** ([backend/src/routes/mod.rs](../backend/src/routes/mod.rs)):

-   `/api/login`, `/api/register` - 认证
-   `/articles`, `/api/article/{id}` - 文章 CRUD
-   `/api/comments`, `/api/comments/{id}` - 评论系统
-   `/api/search/suggestions` - 搜索建议

### 2. Frontend - Vue Web 应用

**入口**: [frontend/src/main.ts](../frontend/src/main.ts)

**技术栈**:

-   Vue 3 Composition API + TypeScript
-   Naive UI (自动导入组件)
-   Pinia (状态管理)
-   Vditor (Markdown 编辑器)
-   MSW 2.x (API mock)

**开发命令**:

```bash
cd frontend
npm install
npm run dev  # Web 开发模式 (Vite + MSW)
```

**API 策略** ([frontend/src/api/client.ts](../frontend/src/api/client.ts)):

-   **Web 环境**: Axios 拦截器自动添加 `Bearer token`
-   **Tauri 环境**: 请求被重定向到 Rust commands (见下文)

**Mock 数据** ([frontend/src/mocks/handlers.ts](../frontend/src/mocks/handlers.ts)):

-   MSW 拦截 `/api/*` 和 `/articles` 请求
-   添加新 API 时同时更新 handlers

### 3. Tauri 桌面应用

**入口**: [frontend/src-tauri/src/lib.rs](../frontend/src-tauri/src/lib.rs)

**与 backend 的关系**：

-   复用 backend 的数据模型和迁移文件 (复制到 `src-tauri/migrations/`)
-   **不共享代码**，独立编译
-   使用相同的 SQLite schema，但数据库文件路径不同

**开发命令**:

```bash
cd frontend
npm run tauri:dev  # 启动 Tauri 开发环境
npm run tauri:build  # 打包桌面应用
```

**核心机制** - HTTP 请求代理:

-   前端调用 `client.get()` 时，Tauri 环境会被 [commands/http.rs](../frontend/src-tauri/src/commands/http.rs) 拦截
-   Token 通过 `__token` 字段传递（非标准 HTTP header）
-   Rust command 转发到内嵌的 backend 逻辑 (repository 层)

```typescript
// 前端代码不需要区分环境，统一使用 client
import client from "@/api/client";
const articles = await client.get("/articles");
```

**Tauri 命令模块** ([frontend/src-tauri/src/commands/](../frontend/src-tauri/src/commands/)):

-   `http.rs` - HTTP 代理 (转发到 backend API 或直接调用 repository)
-   `auth.rs` - 登录/注册/验证
-   `articles.rs`, `comments.rs`, `users.rs` - 各功能模块
-   `mod.rs` - 导出所有命令，在 [lib.rs](../frontend/src-tauri/src/lib.rs) 注册

## 开发工作流

### 添加新功能 (示例：新增"标签"功能)

1. **Backend** - 创建 API:

    - 迁移: `backend/migrations/0006_create_tags.sql`
    - 模型: `backend/src/models/tag.rs`
    - 路由: `backend/src/routes/tags.rs`

2. **Frontend** - 前端实现:

    - 类型: `frontend/src/types/tag.ts` (使用 Zod)
    - API: `frontend/src/api/tag.ts`
    - Mock: `frontend/src/mocks/handlers.ts` 添加 `http.get("/api/tags")`

3. **Tauri** - 桌面应用支持:
    - 迁移: 复制 `0006_create_tags.sql` 到 `src-tauri/migrations/`
    - 模型: `src-tauri/src/models/tag.rs`
    - Repository: `src-tauri/src/repositories/tag.rs`
    - Command: `src-tauri/src/commands/tags.rs`
    - 注册: 在 [lib.rs](../frontend/src-tauri/src/lib.rs) `invoke_handler!` 添加命令

### 数据库迁移

**Backend**:

-   在 `backend/migrations/` 创建 `XXXX_description.sql`
-   重启 `cargo run` 自动应用

**Tauri**:

-   复制到 `src-tauri/migrations/` (保持文件名一致)
-   重启 Tauri 应用自动应用

**种子数据**: `seeds/0001_superuser.sql` 创建默认管理员 (用户名 `admin`)

### 调试技巧

**Backend 日志**:

```bash
RUST_LOG=debug cargo run  # 查看所有 tracing 日志
```

**Frontend**:

-   Web: 浏览器控制台 + Vue DevTools
-   Tauri: 查看终端 Rust 日志 (`log::info!`) + 浏览器控制台

**数据库检查**:

-   Backend: `backend/app.db`
-   Tauri: `%APPDATA%/com.tauri.dev/app.db` (Windows)

## 关键约定

### 认证流程

1. **登录**: POST `/api/login` 返回 JWT token
2. **存储**: [frontend/src/stores/user.ts](../frontend/src/stores/user.ts) 保存到 localStorage
3. **使用**:
    - Web: Axios 拦截器自动添加 `Authorization: Bearer {token}`
    - Tauri: 通过 `__token` 字段传递给 Rust command

### 路由守卫

[frontend/src/router/index.ts](../frontend/src/router/index.ts):

-   `meta: { requiresAdmin: true }` 检查 `user.identity === 'admin'`
-   未授权重定向到 `/login`

### 类型安全

-   **Zod 验证**: 所有 API 响应通过 Zod schema 校验 (如 [types/article.ts](../frontend/src/types/article.ts))
-   **Rust 类型对应**: `backend/src/models/` 和 `frontend/src-tauri/src/models/` 保持一致

### Markdown 编辑器

-   **编辑器**: [components/MarkdownEditor.vue](../frontend/src/components/MarkdownEditor.vue) (Vditor)
-   **预览**: [components/MdPreview.vue](../frontend/src/components/MdPreview.vue)
-   **唯一 ID**: 使用 `uuidv7()` 避免缓存冲突
-   **初始化**: 必须在 `onMounted` 后创建实例

### 环境检测

```typescript
import { useAppStore } from "@/stores/app";
const appStore = useAppStore();
if (appStore.isTauri) {
    // Tauri 特定逻辑
}
```

## 开发注意事项

1. **三层架构同步**:

    - 添加新功能时记得在 backend, frontend mock, Tauri commands 三处实现
    - 迁移文件需同步到 `src-tauri/migrations/`

2. **Token 传递机制**:

    - Tauri 环境使用 `__token` 字段传递 token (见 `frontend/src-tauri/src/commands/http.rs`)
    - Web 环境通过 Axios 拦截器自动添加 Authorization header

3. **数据库路径**:

    - Backend: `database_url` 环境变量
    - Tauri: `Config::get_database_path()` 获取平台路径

4. **MSW 开发提示**:

    - 更新 handlers 后建议硬刷新浏览器 (Ctrl+Shift+R)
    - Service Worker 可能会缓存旧版本

5. **Vditor 使用**:
    - 在 `onMounted` 后初始化编辑器实例
    - 每个实例使用唯一 ID (通过 `uuidv7()` 生成)

## 配置文件

-   [backend/Cargo.toml](../backend/Cargo.toml) - Backend 依赖 (Axum, SQLx)
-   [frontend/src-tauri/Cargo.toml](../frontend/src-tauri/Cargo.toml) - Tauri 依赖 (复用 SQLx 配置)
-   [frontend/package.json](../frontend/package.json) - 前端依赖 + Tauri CLI
-   [frontend/vite.config.ts](../frontend/vite.config.ts) - Vite 配置 (代理、自动导入)
-   [frontend/src-tauri/tauri.conf.json](../frontend/src-tauri/tauri.conf.json) - Tauri 应用配置

## 项目初衷与目标

这是一个**练手学习项目**，主要目的是熟悉技术栈，代码质量和架构设计并不严谨。

**明确说明**：

-   ❌ 不建议用于生产环境或实际项目
-   ❌ 不保证代码质量和最佳实践
-   ❌ 不承担任何使用风险和责任
-   ❌ Web API 随缘维护
-   ✅ 仅供学习参考和技术探索
-   ✅ 欢迎交流讨论和提出建议
-   🚀 未来可能会快速迭代，随时可能大改

**技术展示重点**：

-   Rust Web API 开发 (Axum 框架)
-   Vue 3 + TypeScript 前端工程化
-   Tauri 桌面应用集成
-   SQLite 数据库迁移管理
-   JWT 认证和 Argon2 密码哈希
-   三层架构的实践探索

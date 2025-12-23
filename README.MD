# myBlog

> **⚠️ 项目声明**：这是一个用于学习和练手的项目，代码并不严谨，随缘维护，仅供参考学习。项目不会部署到生产环境，未来可能会快速迭代修改。**不建议直接实际使用**，但欢迎交流探讨！

一个采用三层架构的练习博客项目，展示 Vue 3、Rust 和 Tauri 的集成应用。

## 项目架构

本项目采用**三层架构**设计：

1. **backend/** - 独立 Rust API 服务器 (Axum + SQLite)
    - 可独立运行的 Web API 服务
    - 支持 JWT 认证和 Argon2 密码哈希
2. **frontend/** - Vue 3 Web 应用
    - 开发时使用 MSW (Mock Service Worker) 模拟 API
    - 支持独立的前端开发和测试
3. **frontend/src-tauri/** - Tauri 桌面应用
    - 内嵌完整 Rust 后端逻辑
    - 使用独立的 SQLite 数据库

## 技术栈

### 前端

-   Vue 3
-   TypeScript
-   Naive UI
-   Tailwind CSS
-   Tauri (桌面应用)

### 后端

-   Rust (Axum 框架)
-   SQLite3

## 运行项目

### 前端

```bash
cd frontend
npm install
npm run dev
```

### 后端

```bash
cd backend
# 开发环境
cargo run

# 生产环境
cargo build --release
.\target\release\backend.exe
```

### Tauri 桌面应用

```bash
cd frontend
npm install
npm run tauri dev  # 开发环境
npm run tauri build  # 生产环境打包
```

**📦 下载体验**

已编译的桌面应用可在 [Releases](https://github.com/Jianchi-Chen/myBlog/releases) 页面下载。

## 默认账户

**超级管理员账户**

-   用户名：`admin`
-   密码：`tfF;1J(2WokG,5`

> 适用于 Web 端和 Tauri 桌面应用

## 项目初衷

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
-   Vue 3 Composition API + TypeScript
-   Tauri 2.x 桌面应用开发
-   SQLite 数据库迁移管理
-   JWT 认证和密码安全
-   三层架构实践

## 许可证

暂无许可证。

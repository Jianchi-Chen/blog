# 博客

一个使用 Vue3 和 Rust 构建的练习博客项目。

## 项目结构

-   **frontend**: 使用 Vue 3、TypeScript、Naive UI 和 Tailwind CSS 构建的前端应用
-   **backend**: 使用 Rust (Axum) 和 SQLite3 构建的后端 API
-   **frontend/src-tauri**: 使用 Tauri 构建的桌面应用程序

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

## 说明

这是一个用于学习目的的练习项目。不打算部署到生产环境，有建议欢迎探讨。

## 许可证

暂无许可证。

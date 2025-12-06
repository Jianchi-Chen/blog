# Tauri Auth 模块使用说明

## 概述

本模块为 Tauri 应用提供完整的认证和 HTTP 请求功能，包括：
- JWT token 生成和验证
- 密码哈希和验证（Argon2）
- 自动 token 管理的 HTTP 请求代理
- 用户登录/注册功能

## 架构设计

### 前端（Vue + TypeScript）
前端通过 `client.ts` 统一处理所有 API 请求：
- **浏览器环境**：使用 axios + Bearer token
- **Tauri 环境**：使用 `invoke("http_request")` + `__token` 字段

### 后端（Rust + Tauri）
Tauri 后端提供以下命令：

#### 1. HTTP 请求代理
```rust
http_request(request: HttpRequest) -> Result<Value, String>
```
自动将前端请求转发到后端 API，并自动携带 token。

#### 2. 认证命令

**登录**
```rust
login(credentials: LoginRequest) -> Result<LoginResponse, String>
```

**注册**
```rust
register(user_info: RegisterRequest) -> Result<LoginResponse, String>
```

**验证 Token**
```rust
verify_token(token: String) -> Result<Claims, String>
```

**获取当前用户**
```rust
get_current_user(token: String) -> Result<Value, String>
```

## 前端调用示例

### 使用方式一：通过 client.ts（推荐）

```typescript
import client from '@/api/client'

// 登录
const response = await client.post('/api/login', {
  username: 'admin',
  password: 'password'
})

// 获取文章列表（自动携带 token）
const articles = await client.get('/articles', {
  params: { identity: 'public' }
})
```

在 Tauri 环境下，这些请求会自动通过 `http_request` command 处理。

### 使用方式二：直接调用 Tauri 命令

```typescript
import { invoke } from '@tauri-apps/api/core'

// 登录
const result = await invoke('login', {
  credentials: {
    username: 'admin',
    password: 'password'
  }
})

// 验证 token
const claims = await invoke('verify_token', {
  token: 'your-jwt-token'
})
```

## 配置说明

### 开发环境
在 `src-tauri/.env` 中配置：
```env
DATABASE_URL=./app.db
HOST=127.0.0.1
PORT=3000
JWT_SECRET=your-secret-key
JWT_TTL=604800
```

### 生产环境
首次启动时自动生成配置文件，保存在：
- Windows: `%APPDATA%\{app-identifier}\config.json`
- macOS: `~/Library/Application Support/{app-identifier}/config.json`
- Linux: `~/.config/{app-identifier}/config.json`

配置示例：
```json
{
  "database_url": "./app.db",
  "host": "127.0.0.1",
  "port": 3000,
  "jwt_secret": "auto-generated-uuid",
  "jwt_ttl": 604800
}
```

## 数据库迁移

数据库表结构应在 `src-tauri/migrations/` 目录下定义。

用户表示例（`0002_create_users.sql`）：
```sql
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    email TEXT,
    role TEXT NOT NULL DEFAULT 'user',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## 安全特性

1. **密码安全**：使用 Argon2 算法哈希密码
2. **JWT 验证**：所有需要认证的请求都会验证 token
3. **自动过期**：token 有过期时间（默认 7 天）
4. **随机密钥**：生产环境自动生成 UUID 作为 JWT 密钥

## 注意事项

1. **环境检测**：`client.ts` 使用 `isTauri()` 自动检测运行环境
2. **错误处理**：所有命令都返回 `Result<T, String>`，前端需处理错误
3. **数据库路径**：生产环境数据库自动保存在应用数据目录
4. **CORS**：Tauri 环境下不存在 CORS 问题，请求直接发送到后端

## 文件结构

```
src-tauri/src/
├── auth/
│   └── mod.rs           # JWT 和密码处理
├── commands/
│   ├── mod.rs           # 命令模块导出
│   ├── auth.rs          # 认证相关命令
│   └── http.rs          # HTTP 请求代理
├── config.rs            # 配置管理
├── db.rs                # 数据库连接
└── lib.rs               # 应用入口
```

## 扩展开发

### 添加新的 API 命令

1. 在 `commands/` 下创建新模块
2. 实现 `#[tauri::command]` 函数
3. 在 `commands/mod.rs` 中导出
4. 在 `lib.rs` 的 `invoke_handler` 中注册

示例：
```rust
#[tauri::command]
pub async fn my_command(
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<String, String> {
    // 实现逻辑
    Ok("success".to_string())
}
```

# 配置 GitHub Secrets 指南

## 🔐 密钥已生成

密钥文件位置：
- **私钥**：`frontend/myapp.key`（⚠️ 保密！）
- **公钥**：`frontend/myapp.key.pub`（已更新到 `tauri.conf.json`）

## 📝 下一步操作

### 1. 配置 GitHub Secrets

前往你的 GitHub 仓库：`https://github.com/Jianchi-Chen/blog/settings/secrets/actions`

点击 **New repository secret**，添加以下两个密钥：

#### Secret 1: TAURI_SIGNING_PRIVATE_KEY

**Name**: `TAURI_SIGNING_PRIVATE_KEY`

**Value**: 复制下面的私钥内容（包括所有行）：

```
生成时输出的私钥
```

#### Secret 2: TAURI_SIGNING_PRIVATE_KEY_PASSWORD

**Name**: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

**Value**: 你在生成密钥时输入的密码

---

### 2. 安全移动密钥文件

⚠️ **重要**：密钥文件不应该保存在项目目录中！

**Windows 推荐操作**：

```powershell
# 创建用户主目录下的 .tauri 文件夹
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.tauri"

# 移动密钥文件
Move-Item "frontend\myapp.key" "$env:USERPROFILE\.tauri\myapp.key"
Move-Item "frontend\myapp.key.pub" "$env:USERPROFILE\.tauri\myapp.key.pub"

# 验证文件已移动
Get-ChildItem "$env:USERPROFILE\.tauri"
```

**macOS/Linux 推荐操作**：

```bash
# 创建 .tauri 文件夹
mkdir -p ~/.tauri

# 移动密钥文件
mv frontend/myapp.key ~/.tauri/myapp.key
mv frontend/myapp.key.pub ~/.tauri/myapp.key.pub

# 验证文件已移动
ls -la ~/.tauri
```

---

### 3. 验证配置

✅ 检查清单：

- [ ] GitHub Secret `TAURI_SIGNING_PRIVATE_KEY` 已添加
- [ ] GitHub Secret `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` 已添加
- [ ] 密钥文件已从项目目录移走
- [ ] 公钥已更新到 `frontend/src-tauri/tauri.conf.json`
- [ ] `.gitignore` 已添加 `*.key` 和 `*.key.pub`

---

### 4. 测试发布

```bash
# 提交配置更改
git add .
git commit -m "chore: update updater public key"

# 创建测试版本
git tag v0.1.1
git push origin main --tags

# 前往 GitHub Actions 查看构建进度
# https://github.com/Jianchi-Chen/blog/actions
```

---

## 🔒 安全提示

1. **永远不要提交私钥到 Git**
2. **定期备份私钥和密码**（存储在安全的密码管理器中）
3. **如果私钥泄露，立即重新生成并更新所有配置**
4. **GitHub Secrets 只有仓库管理员可见**

---

## 🆘 常见问题

### Q: 如果忘记密码怎么办？

A: 密码无法恢复，需要重新生成密钥对并更新所有配置。

### Q: 如何备份密钥？

A: 将 `~/.tauri/myapp.key` 和密码保存到安全的密码管理器（如 1Password、Bitwarden）。

### Q: 多台电脑如何共享密钥？

A: 从密码管理器中复制密钥文件到其他电脑的 `~/.tauri/` 目录。

---

**配置完成后可以删除本文档**（建议保留密钥位置信息）

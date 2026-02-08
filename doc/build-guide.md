# 构建与发布指南

## 概述

本项目使用 GitHub Actions 在云端构建 macOS 和 Windows 安装包。
你不需要一台 Windows 电脑，GitHub 免费提供 Windows 构建环境。

---

## 第一步：推送代码到 GitHub

如果仓库还没有关联远程：

```bash
# 在项目根目录执行
git remote add origin https://github.com/<你的用户名>/cc-md-preview.git
git add -A
git commit -m "feat: add CI workflow for multi-platform build"
git push -u origin main
```

如果已有远程仓库，直接 push 即可：

```bash
git add -A
git commit -m "feat: add CI workflow for multi-platform build"
git push
```

---

## 第二步：在 GitHub 上手动触发构建

1. 打开你的仓库页面 `https://github.com/<你的用户名>/cc-md-preview`
2. 点击顶部 **Actions** 标签页
3. 左侧找到 **Build & Release**
4. 点击右侧 **Run workflow** 按钮
5. 选择分支（默认 main），点击绿色 **Run workflow**

构建大约需要 5-10 分钟。

---

## 第三步：下载构建产物

构建完成后（每个 Job 显示绿色 ✓）：

1. 点击刚完成的 workflow run
2. 页面底部 **Artifacts** 区域会列出：
   - **macOS-arm64** — Apple Silicon Mac 的 `.dmg`
   - **macOS-x64** — Intel Mac 的 `.dmg`
   - **Windows-x64** — Windows 的 `.msi` 和 `.exe` 安装包
3. 点击对应的名字下载 zip 文件，解压后就是安装包

---

## 第四步：分享给别人

- **Mac 用户** → 发送 `.dmg` 文件，双击挂载后拖入 Applications
- **Windows 用户** → 发送 `.msi`（静默安装）或 `.exe`（NSIS 安装向导）

### Windows 安装注意事项

由于安装包没有代码签名，Windows 用户安装时会看到 SmartScreen 警告：

> "Windows 已保护你的电脑"

告诉他们点击 **更多信息 → 仍要运行** 即可。

---

## 补充说明

### 构建配额

GitHub 免费账户每月有 2000 分钟 Actions 额度：
- Linux/macOS 任务按 1x 计费
- Windows 任务按 2x 计费

一次完整构建（3 平台）大约消耗 20-30 分钟配额，足够每月构建几十次。

### 只构建 Windows

如果只需要 Windows 产物，可以编辑 `.github/workflows/release.yml`，
把 `matrix.include` 里的两个 macOS 条目删除，只保留 Windows 那一条。

### 本地构建（如果你有 Windows 电脑）

```powershell
# 1. 安装依赖（一次性）
#    - Node.js 18+: https://nodejs.org
#    - Rust: https://rustup.rs
#    - pnpm: npm install -g pnpm
#    - Visual Studio Build Tools（Rust 会提示安装）

# 2. 克隆仓库
git clone https://github.com/<你的用户名>/cc-md-preview.git
cd cc-md-preview

# 3. 构建
pnpm install
pnpm tauri build

# 4. 产物在 src-tauri/target/release/bundle/ 下
#    - msi/  → MSI 安装包
#    - nsis/ → EXE 安装包
```

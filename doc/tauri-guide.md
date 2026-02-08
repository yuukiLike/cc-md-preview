# Tauri 入门指南

## 日常开发

开发模式，边改代码边看效果（热更新）：

```bash
pnpm tauri dev
```

这会同时启动前端 Vite 开发服务器和 Rust 后端，弹出应用窗口。

## 打包发布

### 构建安装包

```bash
# 构建所有格式（.app + .dmg）
pnpm tauri build

# 只构建 .dmg（推荐用于分发）
pnpm tauri build --bundles dmg

# 只构建 .app（自己用）
pnpm tauri build --bundles app
```

### 产物路径

构建完成后，产物在 `src-tauri/target/release/bundle/` 下：

```
src-tauri/target/release/bundle/
├── macos/
│   └── cc-md-preview.app        ← 应用本体
└── dmg/
    └── cc-md-preview_0.1.0_aarch64.dmg  ← 安装包
```

### .app vs .dmg

| | .app | .dmg |
|---|---|---|
| 本质 | 文件夹，macOS 应用本体 | 磁盘镜像，压缩安装包 |
| 分发 | 不方便传输 | 单文件，适合下载分发 |
| 安装 | 双击直接运行 | 打开后拖拽 .app 到 Applications |
| 体积 | 原始大小 | 压缩过，更小 |

简单说：**.dmg 里面装的就是 .app**，它只是一个分发容器。

- 自己用 → 直接跑 `.app`
- 发给别人 → 给 `.dmg`

## 项目结构概览

```
cc-md-preview/
├── src/                  ← 前端代码（Vue 3 + TypeScript）
│   ├── modules/          ← 功能模块
│   ├── stores/           ← Pinia 状态管理
│   └── styles/           ← CSS 样式 & 主题
├── src-tauri/            ← Rust 后端（Tauri）
│   ├── src/
│   │   ├── lib.rs        ← 命令注册入口
│   │   ├── commands/     ← Tauri 命令（前端可调用）
│   │   └── platform/     ← 平台相关代码（macOS/Windows）
│   ├── Cargo.toml        ← Rust 依赖
│   └── tauri.conf.json   ← Tauri 配置（窗口、权限等）
└── package.json          ← 前端依赖
```

### 前端 → 后端通信

前端通过 `invoke` 调用 Rust 命令：

```ts
import { invoke } from "@tauri-apps/api/core";

// 调用 Rust 端的 get_cache_info 命令
const info = await invoke<CacheInfo>("get_cache_info", {
  browser: "Chrome",
});
```

Rust 端用 `#[tauri::command]` 定义命令：

```rust
#[tauri::command]
pub async fn get_cache_info(browser: Option<String>) -> Result<CacheInfo, String> {
    // ...
}
```

然后在 `lib.rs` 注册：

```rust
.invoke_handler(tauri::generate_handler![
    commands::chrome_cache::get_cache_info,
])
```

## 常用命令速查

| 命令 | 用途 |
|------|------|
| `pnpm tauri dev` | 开发模式运行 |
| `pnpm tauri build` | 构建发布包 |
| `pnpm tauri build --bundles dmg` | 只构建 .dmg |
| `pnpm dev` | 只启动前端（不启动 Rust） |
| `cargo build` | 只编译 Rust（在 src-tauri/ 下执行） |
| `pnpm run build` | 只构建前端（类型检查 + Vite 打包） |

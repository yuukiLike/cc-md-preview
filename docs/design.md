# Technical Design

## Architecture Overview

Tauri v2 双进程模型：

```
┌─────────────────────────────────┐
│           WebView               │
│  ┌───────────────────────────┐  │
│  │        Frontend           │  │
│  │  (HTML/CSS/JS Framework)  │  │
│  └───────────┬───────────────┘  │
│              │ IPC (invoke)     │
│              │ Events           │
└──────────────┼──────────────────┘
               │
┌──────────────┼──────────────────┐
│  ┌───────────▼───────────────┐  │
│  │      Core Process         │  │
│  │         (Rust)            │  │
│  │  ┌─────────┐ ┌────────┐  │  │
│  │  │Commands │ │ State  │  │  │
│  │  └─────────┘ └────────┘  │  │
│  │  ┌─────────┐ ┌────────┐  │  │
│  │  │Plugins  │ │  IPC   │  │  │
│  │  └─────────┘ └────────┘  │  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
```

## Frontend

### 框架选择

<!-- 选择前端框架及理由。记录到 decisions/ 目录 -->

- **Framework**:
- **Router**:
- **State Management**:
- **UI Library**:

### 组件结构

```
src/
├── components/     # 通用组件
├── pages/          # 页面级组件
├── stores/         # 状态管理
├── lib/            # 工具函数
└── styles/         # 全局样式
```

## Backend (Rust)

### Commands

<!-- 暴露给前端的 Tauri Commands -->

| Command | 说明 | 参数 | 返回值 |
|---------|------|------|--------|
| | | | |

### State Management

<!-- Tauri managed state，通过 `app.manage()` 注入 -->

```rust
// 示例
struct AppState {
    // ...
}
```

### Plugins

<!-- 使用的 Tauri 官方/社区插件 -->

| Plugin | 用途 | 平台支持 |
|--------|------|---------|
| `tauri-plugin-shell` | | Desktop |
| `tauri-plugin-store` | | All |
| `tauri-plugin-dialog` | | All |

## IPC Contract

前后端通信的接口契约。这是最关键的文档之一 — 前后端的边界在这里定义。

<!-- 每个 command 一行，保持表格更新 -->

| Command Name | Args | Return Type | Error Type | 说明 |
|-------------|------|-------------|------------|------|
| | | | | |

### Event 通道

<!-- 后端向前端推送的事件 -->

| Event Name | Payload | Direction | 说明 |
|-----------|---------|-----------|------|
| | | Core → WebView | |

## Data Model

<!-- 核心数据结构定义，Rust struct 为准 -->

```rust
// 示例
// #[derive(Debug, Serialize, Deserialize)]
// struct Item {
//     id: String,
//     name: String,
//     created_at: DateTime<Utc>,
// }
```

## Platform Specifics

### 桌面 vs 移动端差异

| 能力 | Desktop | Mobile | 处理策略 |
|------|---------|--------|---------|
| 文件系统访问 | 完整 | 受限（沙盒） | |
| 窗口管理 | 多窗口 | 单窗口 | |
| 系统托盘 | 支持 | 不支持 | |
| 通知 | 系统通知 | 推送通知 | |
| 快捷键 | 支持 | 不支持 | |

### 平台特定配置

<!-- 记录各平台需要的特殊配置 -->

- **macOS**:
- **Windows**:
- **Linux**:
- **iOS**:
- **Android**:

## Security

### Capabilities & Permissions

Tauri v2 使用基于 capability 的权限模型。在 `src-tauri/capabilities/` 中定义。

```json
// 示例: src-tauri/capabilities/default.json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default"
  ]
}
```

### 权限策略

<!-- 最小权限原则：只开放需要的能力 -->

| 权限 | 是否需要 | 理由 |
|------|---------|------|
| `fs:read` | | |
| `fs:write` | | |
| `shell:open` | | |
| `dialog:open` | | |
| `http:default` | | |

### CSP 策略

<!-- Content Security Policy 配置 -->

```
default-src 'self'; script-src 'self'
```

## File Structure

```
project-root/
├── src/                    # 前端源码
│   ├── components/
│   ├── pages/
│   ├── stores/
│   ├── lib/
│   └── App.{tsx,vue,svelte}
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs          # 入口、Command 注册
│   │   └── commands/       # Command 实现
│   ├── capabilities/       # 权限配置
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                   # 项目文档（你在这里）
├── package.json
└── README.md
```

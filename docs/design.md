# Technical Design

## Architecture Overview

Tauri v2 双进程模型：

```
┌─────────────────────────────────┐
│           WebView               │
│  ┌───────────────────────────┐  │
│  │        Frontend           │  │
│  │  Vue 3 + Tailwind CSS v4  │  │
│  │  markdown-it + hljs       │  │
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
│  │  │Commands │ │Platform│  │  │
│  │  │ - md    │ │ Paths  │  │  │
│  │  │ - cache │ │        │  │  │
│  │  └─────────┘ └────────┘  │  │
│  │  ┌─────────┐ ┌────────┐  │  │
│  │  │Plugins  │ │ Error  │  │  │
│  │  │ dialog  │ │Handler │  │  │
│  │  │ fs      │ │        │  │  │
│  │  └─────────┘ └────────┘  │  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
```

**关键设计决策：**
- **MD 渲染在前端完成** — 实时预览需要快速反馈，Rust 只负责文件 I/O
- **缓存路径硬编码 + `#[cfg]` 条件编译** — 不接受前端传入路径，防止路径穿越攻击
- **主题通过 CSS class 切换** — 简单无依赖，配合 CSS 变量

## Tech Stack

| 层       | 技术                        | 版本   | 选择理由 |
|----------|---------------------------|--------|---------|
| 框架     | Tauri v2                   | ^2.0   | 轻量原生，Rust 安全性 |
| 前端     | Vue 3 (Composition API)    | ^3.5   | 渐进式，学习曲线平缓 |
| 构建     | Vite                       | ^6     | 快速 HMR |
| 类型     | TypeScript                 | ^5.6   | 类型安全 |
| 样式     | Tailwind CSS v4            | ^4     | 零配置，原子化 CSS |
| 路由     | Vue Router                 | ^4     | Vue 官方路由 |
| 状态     | Pinia                      | ^2     | Vue 官方状态管理 |
| MD 解析  | markdown-it                | ^14    | 默认安全（HTML 转义），防 XSS |
| 代码高亮 | highlight.js               | ^11    | 比 shiki 快 44x，bundle 更小 |
| 包管理   | pnpm                       | 9.7.0  | 快速，磁盘效率高 |

## Frontend

### 框架选择

- **Framework**: Vue 3 (Composition API) — 见 [ADR-001](decisions/001-frontend-framework.md)
- **Router**: Vue Router v4
- **State Management**: Pinia v2
- **UI Library**: Tailwind CSS v4（原子化，不引入组件库）

### 组件结构

```
src/
├── App.vue
├── main.ts
├── styles/
│   ├── main.css                  # @import "tailwindcss"
│   └── themes/                   # MD 主题 CSS
│       ├── github.css
│       ├── github-dark.css
│       ├── minimal.css
│       └── solarized.css
├── router/index.ts
├── stores/                       # Pinia stores
│   ├── markdown.ts
│   └── chrome-cache.ts
├── composables/                  # 通用 composables
├── components/                   # 通用组件
│   ├── layout/
│   │   ├── AppLayout.vue
│   │   ├── AppSidebar.vue
│   │   └── AppHeader.vue
│   └── common/
├── modules/
│   ├── markdown/                 # Markdown 模块
│   │   ├── views/
│   │   │   └── MarkdownView.vue
│   │   ├── components/
│   │   │   ├── MarkdownToolbar.vue
│   │   │   ├── ThemeSelector.vue
│   │   │   ├── MarkdownPreview.vue
│   │   │   └── DropZone.vue
│   │   ├── composables/
│   │   │   ├── useMarkdownRenderer.ts
│   │   │   └── useFileDrop.ts
│   │   └── index.ts
│   └── chrome-cache/             # Chrome Cache 模块
│       ├── views/
│       │   └── ChromeCacheView.vue
│       ├── components/
│       │   ├── CacheSummaryCard.vue
│       │   ├── CacheBreakdown.vue
│       │   ├── CacheEntryList.vue
│       │   └── CleanupButton.vue
│       ├── composables/
│       ├── types.ts
│       └── index.ts
└── types/                        # 全局 TS 类型
    └── modules.ts
```

### 模块注册机制

```typescript
// ModuleDefinition 接口
interface ModuleDefinition {
  id: string
  name: string
  icon: string           // SVG icon 组件名
  route: RouteRecordRaw
  order: number          // 侧边栏排序
}
```

每个模块目录下的 `index.ts` 导出 `ModuleDefinition`，由 `router/index.ts` 统一注册。
新增模块只需：创建目录 → 导出定义 → 在路由中 import。

## Backend (Rust)

### Commands

| Command | 说明 | 参数 | 返回值 |
|---------|------|------|--------|
| `read_markdown_file` | 读取 .md 文件内容 | `path: String` | `Result<String>` |
| `open_markdown_dialog` | 打开文件选择对话框 | — | `Result<Option<String>>` |
| `get_cache_info` | 获取缓存分类大小 | — | `Result<CacheInfo>` |
| `list_cache_entries` | 列出指定类别缓存文件 | `cache_type: String` | `Result<Vec<CacheEntry>>` |
| `clean_cache` | 清理指定类别缓存 | `cache_types: Vec<String>` | `Result<CleanResult>` |
| `get_chrome_profiles` | 获取 Chrome 配置文件列表 | — | `Result<Vec<String>>` |

### Rust 模块结构

```
src-tauri/src/
├── main.rs                       # Tauri 入口
├── lib.rs                        # 命令注册
├── error.rs                      # 统一错误类型 (AppError)
├── commands/
│   ├── mod.rs
│   ├── markdown.rs               # Markdown 文件读取
│   └── chrome_cache.rs           # Chrome 缓存管理
└── platform/
    ├── mod.rs
    └── cache_paths.rs            # 平台感知的缓存路径
```

### Error Handling

统一错误类型 `AppError`，实现 `Into<InvokeError>`：

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
```

### Plugins

| Plugin | 用途 | 平台支持 |
|--------|------|---------|
| `tauri-plugin-dialog` | 文件选择对话框 | Desktop |
| `tauri-plugin-fs` | 文件系统访问 | Desktop |

### Cargo Dependencies

| Crate | 用途 |
|-------|------|
| `walkdir` | 递归遍历缓存目录 |
| `dirs` | 获取平台标准目录 |
| `chrono` | 时间戳格式化 |
| `serde` / `serde_json` | 序列化 |

## IPC Contract

前后端通信的接口契约。

| Command Name | Args | Return Type | Error Type | 说明 |
|-------------|------|-------------|------------|------|
| `read_markdown_file` | `{ path: string }` | `string` | `AppError` | 读取 MD 文件原文 |
| `open_markdown_dialog` | — | `string \| null` | `AppError` | 系统文件选择对话框 |
| `get_cache_info` | — | `CacheInfo` | `AppError` | 缓存分类大小汇总 |
| `list_cache_entries` | `{ cache_type: string }` | `CacheEntry[]` | `AppError` | 列出指定类别缓存文件 |
| `clean_cache` | `{ cache_types: string[] }` | `CleanResult` | `AppError` | 删除指定类别缓存 |
| `get_chrome_profiles` | — | `string[]` | `AppError` | Chrome 配置文件列表 |

### TypeScript 类型定义

```typescript
interface CacheInfo {
  total_size: number           // 总大小 (bytes)
  categories: CacheCategory[]
  last_scanned: string         // ISO 8601
}

interface CacheCategory {
  name: string                 // "Cache" | "Code Cache" | "GPUCache" | ...
  size: number                 // bytes
  file_count: number
  path: string                 // 展示用，不传回后端
}

interface CacheEntry {
  name: string
  size: number
  modified: string             // ISO 8601
}

interface CleanResult {
  freed_bytes: number
  failed_items: string[]
}
```

### Event 通道

暂无后端主动推送事件的需求。未来如果缓存扫描需要进度反馈，可添加：

| Event Name | Payload | Direction | 说明 |
|-----------|---------|-----------|------|
| `cache:scan-progress` | `{ scanned: number, total: number }` | Core → WebView | 扫描进度（预留） |

## Platform Specifics

### Chrome 缓存路径

| 平台 | 路径 |
|------|------|
| macOS | `~/Library/Caches/Google/Chrome/` |
| Windows | `%LOCALAPPDATA%\Google\Chrome\User Data\` |

使用 `#[cfg(target_os)]` 条件编译，路径硬编码在 Rust 端，不由前端传入。

### 平台特定配置

- **macOS**: 使用 `~/Library/Caches/` 路径；窗口 titlebar 使用原生样式
- **Windows**: 使用 `%LOCALAPPDATA%` 路径；需处理长路径问题

## Security

### Capabilities & Permissions

```json
// src-tauri/capabilities/default.json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for CC MD Preview",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:default",
    "fs:default"
  ]
}
```

### 权限策略

| 权限 | 是否需要 | 理由 |
|------|---------|------|
| `fs:read` | 是 | 读取 .md 文件内容 |
| `fs:write` | 否 | 不写入文件 |
| `dialog:open` | 是 | 打开文件选择对话框 |
| `shell:open` | 否 | 不需要打开外部程序 |
| `http:default` | 否 | 纯本地应用 |

### 安全措施

1. **路径穿越防护** — Chrome 缓存路径硬编码在 Rust 端，不接受前端传入
2. **XSS 防护** — markdown-it 默认转义 HTML
3. **最小权限** — 只开放必要的 Tauri capabilities
4. **CSP 策略** — `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'`

## File Structure

```
cc-md-preview/
├── src/                              # Vue 3 前端
│   ├── App.vue
│   ├── main.ts
│   ├── styles/
│   │   ├── main.css
│   │   └── themes/
│   ├── router/index.ts
│   ├── stores/
│   ├── composables/
│   ├── components/
│   ├── modules/
│   │   ├── markdown/
│   │   └── chrome-cache/
│   └── types/
├── src-tauri/                        # Rust 后端
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── error.rs
│   │   ├── commands/
│   │   └── platform/
│   ├── capabilities/default.json
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                             # 项目文档
├── package.json
├── vite.config.ts
├── tsconfig.json
└── index.html
```

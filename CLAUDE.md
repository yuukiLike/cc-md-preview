# CC MD Preview

轻量级 Markdown 预览 + 浏览器缓存管理桌面应用。Tauri v2 + Vue 3 + Rust。

## Stack

| 层 | 技术 | 为什么 |
|---|---|---|
| 框架 | Tauri v2 | 原生窗口，Rust 安全性，体积小 |
| 前端 | Vue 3 Composition API + TypeScript | 渐进式，Tauri 官方模板一致 |
| 样式 | Tailwind CSS v4 | 零配置，只需 `@tailwindcss/vite` + `@import "tailwindcss"` |
| MD 渲染 | markdown-it v14 + highlight.js v11 | markdown-it 默认转义 HTML 防 XSS；hljs 比 shiki 快 44x |
| 图表 | Mermaid | 流程图、时序图、甘特图 |
| 状态 | Pinia v2 | Vue 官方 |
| 路由 | Vue Router v4 | Vue 官方 |
| 构建 | Vite 6 + pnpm | Node 18.20.4（Vite 7 不兼容，锁定 Vite 6） |

## Architecture

```
WebView (Vue 3)  ←—IPC invoke—→  Core Process (Rust)
```

MD 渲染在前端完成（实时预览需要快速反馈），Rust 只做文件 I/O 和缓存操作。

### 前端模块结构

```
src/
├── modules/
│   ├── markdown/           # Markdown 预览模块
│   │   ├── views/          # MarkdownView
│   │   ├── components/     # TabBar, Toolbar, Preview, ThemeGrid, DropZone
│   │   ├── composables/    # useMarkdownRenderer, useFileDrop, useMermaid
│   │   └── constants/      # 主题定义
│   └── chrome-cache/       # 缓存管理模块
│       ├── views/          # ChromeCacheView
│       ├── components/     # SummaryCard, Breakdown, EntryList, CleanupButton
│       └── types.ts
├── stores/                 # Pinia stores (markdown.ts, chrome-cache.ts)
├── components/layout/      # AppLayout, AppSidebar, AppHeader
└── styles/themes/          # 10 套主题 CSS
```

新增模块：创建 `src/modules/<name>/` → 导出 `ModuleDefinition` → 在 router 中 import。

```typescript
interface ModuleDefinition {
  id: string
  name: string
  icon: string
  route: RouteRecordRaw
  order: number  // 侧边栏排序
}
```

### Rust 模块结构

```
src-tauri/src/
├── lib.rs                  # 命令注册入口
├── error.rs                # AppError (thiserror)
├── commands/
│   ├── markdown.rs         # read_markdown_file, open_markdown_dialog
│   └── chrome_cache.rs     # get_cache_info, list_cache_entries, clean_cache, get_chrome_profiles
└── platform/
    └── cache_paths.rs      # #[cfg(target_os)] 条件编译，硬编码缓存路径
```

### IPC 命令

| Command | 参数 | 返回 |
|---------|------|------|
| `read_markdown_file` | `{ path: string }` | `string` |
| `open_markdown_dialog` | — | `string \| null` |
| `get_cache_info` | — | `CacheInfo` |
| `list_cache_entries` | `{ cache_type: string }` | `CacheEntry[]` |
| `clean_cache` | `{ cache_types: string[] }` | `CleanResult` |
| `get_chrome_profiles` | — | `string[]` |

## Commands

```bash
pnpm tauri dev          # 开发模式（HMR + Rust 后端）
pnpm tauri build        # 构建安装包（.dmg / .msi）
pnpm dev                # 只启动前端
pnpm run build          # 只构建前端（类型检查 + Vite 打包）
cargo doc --no-deps --open   # 生成 Rust API 文档并在浏览器打开（在 src-tauri/ 下执行）
```

## Conventions

- 模块化：每个功能是 `src/modules/<name>/` 下的独立模块
- Rust 错误统一用 `AppError` enum，实现 `Into<InvokeError>`
- Rust 模块用 `pub mod` 声明，保持 `cargo doc` 可见性
- Rust 公开函数和结构体用 `///` 文档注释（支持 Markdown），内部解释用 `//`
- 缓存路径硬编码在 Rust 端，不接受前端传入（防路径穿越）
- 主题通过 CSS class 切换 + CSS 变量
- Tailwind v4 零配置，不要创建 tailwind.config
- Tauri plugins: `dialog`（文件选择）、`fs`（文件读取）

## Constraints

### Scope
- 只做预览，不做 Markdown 编辑器
- MVP 仅支持 Chrome 缓存（不支持 Firefox/Safari/Edge）
- 纯本地应用，无网络请求，无云同步

### Gotchas
- `tauri_plugin_dialog::FilePath` 是 enum (Path/Url)，不是 struct，没有 `.path` 字段
- `tsconfig.node.json` 需要 `"composite": true` 和 `"noEmit": false`
- Node 18 与 Vite 7 不兼容，必须用 Vite 6
- Chrome 缓存路径：macOS `~/Library/Caches/Google/Chrome/`，Windows `%LOCALAPPDATA%\Google\Chrome\User Data\`

### Security
- markdown-it 默认转义 HTML（防 XSS）
- CSP: `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'`
- 最小 Tauri capabilities: `core:default`, `dialog:default`, `fs:default`
- 不开放 `fs:write`、`shell:open`、`http:default`

## Workflow Habits

- 遇到坑、学到新知识时，写入 `docs/notes.md`
- 做了"做/不做"的决策时，写入 `DEVLOG.md`
- 跨项目通用的知识写入 auto-memory

## Platforms

- macOS (Apple Silicon + Intel)
- Windows (x64)
- GitHub Actions CI 构建三平台安装包

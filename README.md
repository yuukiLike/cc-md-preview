# CC MD Preview

轻量级 Markdown 预览桌面应用，支持多文件标签页、10 种主题切换、Mermaid 图表和代码高亮。

基于 Tauri v2 构建，体积小、启动快、跨平台（macOS / Windows）。

## 功能

- **Markdown 渲染** — 基于 markdown-it，支持 GFM 语法
- **多文件标签页** — 同时打开多个 .md 文件，Tab 键快速切换
- **10 种主题** — GitHub / Solarized / Arctic / Dracula / Nord / Cobalt / Monokai / Terminal / Sunset / GitHub Dark
- **Mermaid 图表** — 流程图、时序图、甘特图等
- **代码高亮** — highlight.js 驱动，自动识别语言
- **拖放打开** — 直接拖拽 .md 文件到窗口
- **Chrome 缓存管理** — 查看和清理 Chrome 浏览器缓存

## 安装

从 [Releases](https://github.com/yuukiLike/cc-md-preview/releases) 下载对应平台的安装包：

| 平台 | 文件 |
|------|------|
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Windows | `.msi` 或 `.exe` |

## 从源码构建

### 环境要求

- Node.js 18+
- Rust 1.77+
- pnpm

### 步骤

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建安装包
pnpm tauri build
```

构建产物在 `src-tauri/target/release/bundle/` 目录下。

## 技术栈

| 层 | 技术 |
|----|------|
| 框架 | Tauri v2 |
| 前端 | Vue 3 + TypeScript |
| 样式 | Tailwind CSS v4 |
| 渲染 | markdown-it + highlight.js + Mermaid |
| 后端 | Rust |
| 构建 | Vite 6 + pnpm |

## 项目结构

```
src/
├── modules/
│   ├── markdown/        # Markdown 预览模块
│   │   ├── components/  # TabBar, Toolbar, Preview, ThemeGrid...
│   │   ├── composables/ # useMarkdownRenderer, useFileDrop, useMermaid...
│   │   ├── constants/   # 主题定义
│   │   └── views/       # MarkdownView
│   └── chrome-cache/    # Chrome 缓存管理模块
├── stores/              # Pinia 状态管理
└── styles/themes/       # 10 套主题 CSS

src-tauri/
├── src/
│   ├── commands/        # Tauri 命令 (文件读取、缓存管理)
│   └── platform/        # 平台相关逻辑 (缓存路径)
└── tauri.conf.json
```

## License

MIT

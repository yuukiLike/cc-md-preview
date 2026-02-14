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

需要 Node.js 18+、Rust 1.77+、pnpm。

```bash
pnpm install
pnpm tauri dev       # 开发模式
pnpm tauri build     # 构建安装包
```

产物在 `src-tauri/target/release/bundle/` 目录下。

### CI 构建

推送到 GitHub 后，通过 Actions → **Build & Release** → Run workflow 触发云端构建，自动生成 macOS (ARM/x64) + Windows 安装包。详见 `.github/workflows/`。

## 技术栈

Tauri v2 · Vue 3 · TypeScript · Tailwind CSS v4 · markdown-it · highlight.js · Mermaid · Rust · Vite 6 · pnpm

## License

MIT

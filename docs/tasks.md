# Tasks

## Current Iteration

<!-- 2026-02-07 ~ -->

### Todo

#### Phase 0: 环境准备
- [P0] 更新 Rust 工具链至 >= 1.77.2（`rustup update stable`）
- [P0] 安装 Tauri CLI v2（`cargo install tauri-cli --version "^2"`）

#### Phase 1: 项目脚手架
- [P0] 用 `pnpm create tauri-app` 生成 Vue + TypeScript 模板
- [P0] 将生成文件移入仓库根目录（保留现有 `/docs/`）
- [P0] 安装 Tailwind v4（`pnpm add -D tailwindcss @tailwindcss/vite`）
- [P0] 配置 `vite.config.ts`（Vue + Tailwind 插件 + `@` 别名）
- [P0] 安装路由和状态管理（`pnpm add vue-router pinia`）
- [P0] 验证 `pnpm tauri dev` 能启动窗口

#### Phase 2: App Shell
- [P0] 创建 `AppLayout.vue` / `AppSidebar.vue` / `AppHeader.vue`
- [P0] 用 Tailwind 实现极简原生风格侧边栏（固定左侧，可折叠）
- [P0] 配置 Vue Router：`/markdown` 和 `/chrome-cache` 两个路由
- [P1] 实现模块注册机制（`ModuleDefinition` 接口）供未来扩展

#### Phase 3: Markdown 模块 - Rust 后端
- [P0] 创建 `commands/markdown.rs`（`read_markdown_file` + `open_markdown_dialog`）
- [P0] 添加 `tauri-plugin-dialog` 和 `tauri-plugin-fs` 依赖
- [P0] 注册命令到 `lib.rs`，配置 `capabilities/default.json`

#### Phase 4: Markdown 模块 - Vue 前端
- [P0] `useMarkdownRenderer` composable（markdown-it + highlight.js）
- [P0] `useFileDrop` composable（Tauri drag-drop 事件）
- [P0] 组件：`MarkdownToolbar` / `ThemeSelector` / `MarkdownPreview` / `DropZone`
- [P1] 4 套主题 CSS：github / github-dark / minimal / solarized
- [P0] Pinia store 管理文件路径、内容、当前主题

#### Phase 5: Chrome Cache 模块 - Rust 后端
- [P0] 创建 `platform/cache_paths.rs`（平台感知路径）
- [P0] 创建 `commands/chrome_cache.rs`（`get_cache_info` / `list_cache_entries` / `clean_cache` / `get_chrome_profiles`）
- [P0] 添加 Cargo 依赖：`walkdir` / `dirs` / `chrono`

#### Phase 6: Chrome Cache 模块 - Vue 前端
- [P0] 组件：`CacheSummaryCard` / `CacheBreakdown` / `CacheEntryList` / `CleanupButton`
- [P0] 确认对话框（清理前二次确认）
- [P0] Pinia store 管理缓存信息和扫描状态

#### Phase 7: 收尾
- [P1] 空状态和加载动画
- [P1] 错误处理 UI
- [P2] 更新项目文档

### Doing

### Done

---

## Backlog

- [ ] 支持 Linux 平台
- [ ] 缓存扫描进度条
- [ ] Markdown 文件拖拽排列 / 多文件预览
- [ ] 其他浏览器缓存管理（Firefox / Edge）
- [ ] 深色模式全局切换
- [ ] 自动检测系统主题

---

## Archive

<!-- 已完成的迭代，折叠保留 -->

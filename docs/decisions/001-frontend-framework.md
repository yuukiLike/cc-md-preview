# ADR-001: 前端技术栈选择

- **Status**: Accepted
- **Date**: 2026-02-07

## Context

需要为 Tauri v2 桌面应用选择前端技术栈，包括框架、样式方案、Markdown 解析库、代码高亮库。
约束：桌面场景，性能优先，bundle 体积敏感，需要安全的 Markdown 渲染。

## Options Considered

### Option A: Vue 3 + Tailwind CSS v4 + markdown-it + highlight.js

- Pros:
  - Vue 3 Composition API 灵活，学习曲线平缓
  - Tailwind v4 零配置（`@tailwindcss/vite` 插件）
  - markdown-it 默认 HTML 转义，天然防 XSS
  - highlight.js 客户端速度快，bundle 小
- Cons:
  - Tailwind v4 相对较新，社区资源略少

### Option B: React + CSS Modules + marked + shiki

- Pros:
  - React 生态最大
  - shiki 语法高亮更准确（基于 TextMate grammar）
- Cons:
  - marked 默认不转义 HTML，需额外配置防 XSS
  - shiki 比 highlight.js 慢 44 倍，bundle 显著更大
  - 桌面预览场景不需要 shiki 级别的精确度

### Option C: Svelte + UnoCSS + markdown-it + highlight.js

- Pros:
  - Svelte 编译时框架，运行时极小
  - UnoCSS 灵活
- Cons:
  - Svelte 生态较小
  - Tauri 社区 Svelte 模板较少

## Decision

选择 **Option A: Vue 3 + Tailwind CSS v4 + markdown-it + highlight.js**

核心理由：
1. **安全性** — markdown-it 默认安全，开箱即用防 XSS
2. **性能** — highlight.js 在桌面预览场景下速度远优于 shiki
3. **开发体验** — Vue 3 Composition API + Tailwind v4 零配置，开发效率高
4. **轻量** — 整体 bundle 体积小，适合桌面应用

## Consequences

- **Good**: 安全默认值减少安全问题；highlight.js 性能满足实时预览需求
- **Bad**: Tailwind v4 较新，遇到 bug 时社区解决方案可能较少
- **Neutral**: 技术栈与 Tauri 官方模板（Vue + TypeScript）一致，减少配置摩擦

# Project Plan

## Problem

开发者和内容创作者日常频繁打开 .md 文件查看效果，但缺少一个轻量、好看、支持多主题的本地预览工具。同时，Chrome 浏览器缓存长期堆积占用大量磁盘空间，手动清理步骤繁琐且不直观。

## Solution

一个轻量级桌面应用，帮助用户快速预览 Markdown 文件（支持多风格主题切换）并管理 Chrome 浏览器缓存（查看大小明细 + 一键清理）。

## Target Platforms

- [x] macOS
- [x] Windows
- [ ] Linux
- [ ] iOS
- [ ] Android

## MVP Scope

### 做什么（In Scope）

1. **Markdown 预览** — 打开 .md 文件（对话框 + 拖拽），实时渲染，4 套主题切换（github / github-dark / minimal / solarized）
2. **Chrome 缓存管理** — 扫描缓存目录，按类别显示大小明细，支持选择性清理
3. **模块化架构** — 侧边栏导航 + 模块注册机制，支持未来功能扩展

### 不做什么（Out of Scope）

1. Markdown 编辑功能（只做预览，不做编辑器）
2. 其他浏览器缓存管理（MVP 仅支持 Chrome）
3. 云端同步、多设备协作

## Milestones

| # | 里程碑 | 目标 | 状态 |
|---|--------|------|------|
| 0 | 环境准备 | Rust >= 1.77.2 + Tauri CLI v2 | Todo |
| 1 | 项目脚手架搭建完成 | `pnpm tauri dev` 能启动窗口 | Todo |
| 2 | App Shell 完成 | 侧边栏 + 路由导航可用 | Todo |
| 3 | Markdown 模块可用 | 打开文件、渲染、主题切换 | Todo |
| 4 | Chrome Cache 模块可用 | 查看缓存大小、清理 | Todo |
| 5 | MVP 完成 | 收尾打磨、错误处理 | Todo |

## Risks

| 风险 | 影响 | 应对 |
|------|------|------|
| Rust 版本过低不兼容 Tauri v2 | 阻塞全部开发 | Phase 0 先升级 Rust 工具链 |
| Chrome 缓存路径跨版本变化 | 扫描失败 | 硬编码已知路径 + 优雅降级提示 |
| Tailwind v4 生态尚新，插件兼容性 | 样式问题 | 使用官方 @tailwindcss/vite 插件，零配置方案 |
| 缓存清理误删用户数据 | 数据丢失 | 只操作缓存目录 + 清理前二次确认 + 路径硬编码防穿越 |

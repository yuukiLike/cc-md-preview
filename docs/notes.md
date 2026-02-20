# 学习笔记

开发过程中搞懂的技术点。给未来的自己看。

---

## tsconfig 为什么要两个文件

`tsconfig.json` 管浏览器代码（`src/**`），有 DOM 类型、JSX、路径别名。
`tsconfig.node.json` 管 Node 脚本（`vite.config.ts`），有 `composite: true`，没有 DOM。

两个文件对应两个运行环境。合并会导致类型污染——Node 代码拿到 DOM 类型，或浏览器代码丢失 DOM 类型。这是 Vite 官方脚手架的标准结构。

## Tauri 开发/构建调用链路

```
pnpm dev
  → "tauri dev"
    → 1. beforeDevCommand = "pnpm dev:fe" = "vite"
         Vite dev server 启动在 localhost:1420
    → 2. 编译 Rust (src-tauri/src/)
    → 3. 打开原生窗口，WebView 加载 http://localhost:1420

pnpm build
  → "tauri build"
    → 1. beforeBuildCommand = "pnpm build:fe" = "vue-tsc --noEmit && vite build"
         类型检查 + 打包到 dist/
    → 2. 编译 Rust (release mode)
    → 3. 打包成 .dmg / .msi（读取 tauri.conf.json 的 bundle 配置）
```

关键：Tauri 是外层调度者，它先启动前端，再编译 Rust，最后用原生窗口套住前端页面。

## Tailwind CSS v4 零配置

不需要 `tailwind.config.js`。只需要：
1. `@tailwindcss/vite` 插件加到 vite.config.ts
2. CSS 入口文件写 `@import "tailwindcss"`

## markdown-it 为什么不用 marked

marked 默认不转义 HTML，需要额外配置防 XSS。markdown-it 默认安全。桌面应用场景下安全 > 速度。

## highlight.js vs shiki

shiki 基于 TextMate grammar，语法高亮更精确，但比 hljs 慢 44 倍，bundle 大得多。桌面预览场景不需要那个精度。

## Tauri 的 beforeDevCommand 不能指向 tauri dev

`tauri dev` 启动时先执行 `beforeDevCommand` 启动前端 dev server，然后编译 Rust 打开窗口。
如果 `beforeDevCommand` 指向的 script 最终又调用了 `tauri dev`，就会无限递归。
`beforeDevCommand` 必须是纯前端命令（如 `vite`），不能包含 `tauri`。

## tauri_plugin_dialog::FilePath

是 enum（Path / Url），不是 struct。没有 `.path` 字段，要用 match 解构。

## Git Branch vs Tag

**Branch 是路，Tag 是路上的里程碑。**

- Branch：可移动的指针，每次 commit 往前走。用来描述"正在做什么"（`main`、`feat/dark-mode`）
- Tag：钉死的标记，永远指向同一个 commit。用来标记"这个版本发布了"（`v0.1.0`）

不要用版本号做分支名（如 `v0.0.1` 分支），版本号属于 tag。

Tag 用 annotated tag（`git tag -a`），带时间戳、作者、消息，GitHub 会自动关联 Release。

```
main:  A ── B ── C ── D ── E
                 ↑         ↑
              v0.1.0    v0.2.0    ← tag 钉死不动，branch 继续往前走
```

## 个人开发者 Git 工作流

只需要 `main` 一条主线 + 按需开 feature 分支：

```bash
/feat <name>     # 从 main 创建 feat/<name> 分支
# ...开发、commit...
/done            # 合回 main（--no-ff 保留合并记录），删除分支
/release minor   # 更新版本号、打 tag、推送
```

小改动直接在 main 上 commit，不用开分支。

### Tag 规范

- 格式：`v` 前缀 + SemVer（`v0.1.0`）
- GitHub/Tauri CI 默认匹配 `v*`，保持一致
- `patch`（修 bug）、`minor`（加功能）、`major`（大版本/破坏性变更）

### 注意事项

- 删除 tag 会导致关联的 GitHub Release 一起消失，需要重新打包上传
- `--no-ff` 合并能在 `git log --graph` 里保留分支轨迹，方便回溯
- 空分支（没有新 commit）合并没有意义，不要 merge

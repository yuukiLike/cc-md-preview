# Workflow

> Solo 开发不需要流程，需要的是思考的支架。
> 文档不是给别人看的，是给未来的自己看的，也是给 AI 辅助开发时提供上下文的。

## 五个阶段

```
Plan → Design → Build → Ship → Retrospect
```

| 阶段 | 做什么 | 操作哪个文档 |
|------|--------|-------------|
| **Plan** | 想清楚要做什么、不做什么 | `plan.md` |
| **Design** | 想清楚怎么做、技术方案 | `design.md`, `decisions/` |
| **Build** | 写代码、移动任务卡片 | `tasks.md` |
| **Ship** | 发版、记录变更 | `changelog.md` |
| **Retrospect** | 回顾本轮迭代、调整下轮计划 | `plan.md`, `tasks.md` |

## 迭代节奏

以 **周** 为单位：

- **周一**：花 15 分钟回顾 `tasks.md`，把本周要做的任务从 Todo 移到 Doing
- **每天**：写代码，完成的任务移到 Done
- **周五**：花 15 分钟回顾本周，更新 `changelog.md`，规划下周任务

## 文档使用时机

- **项目启动时**：填写 `plan.md` → 填写 `design.md` → 拆任务到 `tasks.md`
- **遇到技术分叉时**：写一份 ADR 到 `decisions/`
- **每次发版时**：更新 `changelog.md`
- **需要 AI 辅助时**：把相关文档喂给 AI 作为上下文

## 文件清单

```
docs/
├── workflow.md              ← 你在这里
├── plan.md                  # 项目规划
├── design.md                # 技术设计（含 Tauri 架构）
├── tasks.md                 # 任务看板
├── decisions/
│   └── 000-template.md      # ADR 模板
└── changelog.md             # 变更日志
```

## 原则

1. **文档是给自己看的** — 不用写得漂亮，写得清楚就行
2. **只在需要时写** — ADR 不强制，遇到分叉才写
3. **保持轻量** — 如果觉得文档是负担，砍掉不需要的部分
4. **AI 友好** — 结构化的 Markdown 方便 AI 理解上下文

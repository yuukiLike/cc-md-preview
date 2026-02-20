---
allowed-tools: Bash
description: 完成功能 - 将当前 feature 分支合回 main 并清理
---

用户完成了当前功能的开发，要把 feature 分支合回 main。

## 步骤

### 1. 检查当前分支

```bash
git branch --show-current
```

- 如果当前在 `main` 上，告诉用户"你已经在 main 上了，不需要合并"并结束。
- 如果当前不是 `feat/` 开头的分支，提醒用户这不是一个 feature 分支，确认是否继续。

### 2. 检查未提交变更

```bash
git status --porcelain
```

如果有未提交的变更，提醒用户先提交，**不要自动 commit**。

### 3. 合并到 main

```bash
git checkout main
git merge --no-ff <当前分支名> -m "merge: <当前分支名> into main"
```

使用 `--no-ff` 保留分支合并记录，这样 `git log --graph` 能看到完整的功能开发轨迹。

### 4. 清理分支

```bash
git branch -d <已合并的分支名>
```

仅删除本地分支。如果远程也有同名分支，提示用户是否要删除远程分支，**不要自动删除**。

### 5. 输出

```
✅ 已合并：<分支名> → main
🗑️ 已删除本地分支：<分支名>
📍 当前位于：main (<hash>)
👉 如果准备发版，用 /release
```

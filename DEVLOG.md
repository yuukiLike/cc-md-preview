# Dev Log

每条记录回答一个问题：做什么，或者不做什么，为什么。
不是 changelog（那是 git log 的事）。是给未来的自己看的决策备忘。

---

## 2026-02-14

- **砍掉 docs/ doc/ report/ 三个文件夹** — 架构信息合并进 CLAUDE.md，Apple 发布指南存入 auto-memory，任务管理用 Claude Code 内置工具。solo dev 不需要团队仪式。
- **不做** Markdown 编辑功能 — 只做预览。编辑器是另一个产品。
- **不做** 多浏览器缓存支持 — MVP 只管 Chrome。Firefox/Edge 等以后再说，也许永远不做。
- **不做** 云同步 — 纯本地工具。简单就是价值。
- **不维护 CHANGELOG.md** — git log + GitHub Releases 就是 changelog。手动维护等于在两个地方记录同一件事，一定会不同步。

# Apple 应用发布指南

> 适用于 macOS 和 iOS 应用发布。本文档覆盖从零开始的完整流程，可跨项目复用。

## 前置条件：Apple Developer Program

**费用：$99/年**（个人或组织均可），一个账号可发布无限数量的 macOS + iOS 应用。

### 注册步骤

1. 访问 [developer.apple.com/programs](https://developer.apple.com/programs/)
2. 用你的 Apple ID 登录（没有就注册一个）
3. 选择 **个人 (Individual)** 类型（Solo 开发选这个就够了）
4. 填写真实姓名、地址，支付 $99
5. 等待 Apple 审核（通常 24-48 小时，偶尔更久）

> **提示：** 不要等到要发布了才注册。审核需要时间，建议开发早期就注册好。

### 你会获得什么

| 能力 | 说明 |
|------|------|
| 代码签名证书 | 让 macOS/iOS 信任你的 app |
| 公证服务 | macOS app 的自动安全扫描 |
| App Store Connect | 上传和管理 App Store 上架的后台 |
| TestFlight | iOS/macOS 测试分发（最多 10000 名测试员） |
| 每年 100 台设备的 Ad Hoc 测试 | iOS 真机调试和分发 |

---

## 核心概念

### 证书 (Certificates)

证书 = Apple 确认"这个 app 确实是你打包的"。有两种：

| 证书类型 | 用途 |
|---------|------|
| **Development** | 开发和调试用 |
| **Distribution** | 发布用（App Store 或直接分发） |

macOS 还有一个特殊的：
- **Developer ID Application** — 用于 App Store **外**直接分发（.dmg / .pkg）

### 签名 (Code Signing)

用你的证书给 app 打上数字签名。作用：
- 证明 app 来源可信（是你打包的，没被篡改）
- macOS Gatekeeper 检查签名决定是否允许运行
- 没有签名 → 用户看到"无法验证开发者"警告

### 公证 (Notarization)

仅 macOS。签名之后的额外步骤：
1. 把签名好的 app 上传到 Apple 服务器
2. Apple 自动扫描（检查恶意软件、验证签名完整性）
3. 通过后返回一个"票据"（ticket），钉回 app
4. 用户打开 app 时，macOS 联网验证票据 → 无警告直接运行

**不是人工审核**，全自动，通常 2-5 分钟完成。

### Provisioning Profile（仅 iOS）

iOS 比 macOS 多一层限制：Provisioning Profile = 证书 + App ID + 设备列表的组合，告诉 iOS "这个 app 可以在哪些设备上运行"。macOS 直接分发不需要这个。

---

## macOS 发布路径

有两条路，根据需求选择：

### 路径 A：直接分发（推荐 Tauri 应用）

不上 App Store，直接给用户下载 `.dmg` 安装。

**优点：** 无审核、无沙盒限制、自由更新节奏
**缺点：** 用户需要手动下载安装，没有 App Store 曝光

#### 完整流程

```
代码 → tauri build → .app → 签名 → 公证 → .dmg → 分发
```

**第一步：创建 Developer ID 证书**

1. 打开 Mac 上的"钥匙串访问"(Keychain Access)
2. 菜单 → 证书助理 → 从证书颁发机构请求证书
3. 填写邮箱，选择"存储到磁盘"，生成 CSR 文件
4. 登录 [developer.apple.com/account](https://developer.apple.com/account)
5. Certificates → 点 "+" → 选择 **Developer ID Application**
6. 上传 CSR → 下载证书 → 双击安装到钥匙串

**第二步：生成 App 专用密码（用于公证）**

1. 访问 [appleid.apple.com](https://appleid.apple.com)
2. 登录 → 安全 → App 专用密码 → 生成
3. 保存密码（形如 `xxxx-xxxx-xxxx-xxxx`），后续公证要用

**第三步：配置 Tauri 签名**

在 `src-tauri/tauri.conf.json` 中：

```jsonc
{
  "bundle": {
    "macOS": {
      "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
      "minimumSystemVersion": "10.15"
    }
  }
}
```

或通过环境变量（推荐，不把身份信息写进代码）：

```bash
# .env.local（不要提交到 git）
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAM_ID)"
```

**第四步：构建 + 签名**

```bash
pnpm tauri build
```

Tauri 会自动用配置的签名身份对 `.app` 签名。

**第五步：公证**

```bash
# 提交公证
xcrun notarytool submit target/release/bundle/macos/YourApp.app.zip \
  --apple-id "your@email.com" \
  --team-id "TEAM_ID" \
  --password "app-specific-password" \
  --wait

# 公证通过后，钉上票据
xcrun stapler staple target/release/bundle/macos/YourApp.app
```

> `--wait` 会等到公证完成再返回，通常 2-5 分钟。

**第六步：打包 .dmg 分发**

Tauri build 默认会生成 `.dmg`，直接分发即可。

#### 自动化公证（CI/CD 推荐）

在 `src-tauri/tauri.conf.json` 中配置环境变量，Tauri build 可以自动完成签名+公证：

```bash
# 设置环境变量后，tauri build 自动公证
export APPLE_ID="your@email.com"
export APPLE_PASSWORD="app-specific-password"  # App 专用密码
export APPLE_TEAM_ID="TEAM_ID"
```

### 路径 B：Mac App Store

上架 App Store，通过苹果商店分发。

**优点：** 曝光量、自动更新、用户信任度高
**缺点：** 需要审核（1-3 天）、严格沙盒限制、Apple 抽成 15-30%

#### 额外限制

- 必须在 App Sandbox 中运行（文件访问受限）
- 不能随意访问文件系统（Chrome 缓存清理功能可能受影响）
- 需要处理 entitlements 权限声明

#### 流程概要

1. 在 App Store Connect 创建 App 记录
2. 用 **Mac App Store Distribution** 证书签名（不是 Developer ID）
3. 用 Xcode 或 `xcrun altool` 上传到 App Store Connect
4. 填写 App 信息（截图、描述、分类、定价）
5. 提交审核 → 等待 1-3 天
6. 审核通过 → 上架

> **对 CC MD Preview 的建议：** Chrome 缓存清理需要访问用户目录，App Store 沙盒会限制这个功能。推荐走路径 A（直接分发）。

---

## iOS 发布路径

iOS 只能通过 App Store 分发（或 TestFlight 测试）。

### 流程

```
代码 → Xcode 构建 → 签名 → 上传 App Store Connect → 审核 → 上架
```

**第一步：在 App Store Connect 创建 App**

1. 登录 [appstoreconnect.apple.com](https://appstoreconnect.apple.com)
2. My Apps → "+" → 新建 App
3. 填写：App 名称、Bundle ID、SKU、主语言

**第二步：配置签名（Xcode 自动管理最简单）**

1. Xcode → 项目设置 → Signing & Capabilities
2. 勾选 "Automatically manage signing"
3. 选择你的 Team（Developer Program 注册的账号）
4. Xcode 自动创建证书和 Provisioning Profile

**第三步：打包上传**

1. Xcode → Product → Archive
2. Archive 完成后 → Distribute App → App Store Connect
3. 上传完成后去 App Store Connect 后台操作

**第四步：填写 App Store 信息**

| 必填项 | 说明 |
|--------|------|
| 截图 | 各尺寸设备各需至少 1 张（6.7"、6.5"、5.5" 等） |
| 描述 | App 功能描述 |
| 关键词 | 搜索关键词（100 字符内） |
| 分类 | 主分类 + 可选副分类 |
| 隐私政策 URL | 必填，即使不收集数据 |
| 年龄分级 | 根据内容填写问卷 |

**第五步：提交审核**

- 选择构建版本 → 提交审核
- 首次审核通常 24-48 小时
- 被拒的话会告诉你原因，修改后可重新提交

### TestFlight 测试（上架前推荐）

1. 上传构建到 App Store Connect
2. 进入 TestFlight 标签页
3. **内部测试：** 添加团队成员（最多 100 人），无需审核
4. **外部测试：** 添加外部测试员（最多 10000 人），需要 Beta 版审核（通常 1 天）

---

## 常见踩坑点

### 证书过期

- 证书有效期通常 5 年，Developer ID 也是
- 到期前 Xcode 会提醒，续期需要重新生成

### Team ID 在哪里找

- 登录 [developer.apple.com/account](https://developer.apple.com/account)
- 页面右上角 → Membership Details → Team ID（10 位字母数字）

### 签名身份名称格式

```
Developer ID Application: 你的名字 (TEAM_ID)
```

可以用命令查看本机已有的签名身份：

```bash
security find-identity -v -p codesigning
```

### 公证失败常见原因

| 原因 | 解决 |
|------|------|
| 未签名的嵌入二进制 | 确保所有 .dylib/.framework 都签了名 |
| 使用了废弃的 API | 更新代码 |
| Hardened Runtime 未启用 | Tauri 默认启用，一般不会遇到 |
| App 专用密码错误 | 重新生成一个 |

### App Store 审核被拒常见原因

| 原因 | 说明 |
|------|------|
| 4.0 Design | UI 太简陋或体验差 |
| 2.1 Performance | 有 bug 或崩溃 |
| 5.1.1 Privacy | 缺少隐私政策或权限说明不清 |
| 4.2 Minimum Functionality | 功能太少，Apple 觉得"没必要做成 app" |

---

## 费用汇总

| 项目 | 费用 | 说明 |
|------|------|------|
| Apple Developer Program | $99/年 | 必须，涵盖所有平台 |
| App Store 抽成 | 15-30% | 仅付费 app / 内购，小企业 < $1M 收入可申请 15% |
| 域名（隐私政策页面） | ~$10/年 | App Store 上架需要隐私政策 URL |

**免费 app 无内购 → 只需 $99/年开发者费用。**

---

## 本项目（CC MD Preview）的发布清单

- [ ] 注册 Apple Developer Program ($99/年)
- [ ] 创建 Developer ID Application 证书
- [ ] 生成 App 专用密码
- [ ] 在 `tauri.conf.json` 中配置签名
- [ ] `pnpm tauri build` 构建
- [ ] 公证（`xcrun notarytool submit`）
- [ ] 测试 .dmg 安装是否无警告
- [ ] 选择分发渠道（GitHub Releases / 个人网站）

# GitHub Actions 自动化指南

完整的自动化编译和发布工作流，支持所有主要平台。

## 📋 工作流文件

### 1. `release.yml` - Release 发布工作流

**触发条件**: 创建 Git tag (例如 `v0.2.0`)

**功能**:

- 自动检测版本号
- 编译 6 个平台的二进制文件
- 生成对应的 Release
- 上传所有构建产物到 Release 页面

**编译目标**:

- ✅ Linux x86_64 (GNU)
- ✅ Linux ARM64 (GNU)
- ✅ macOS x86_64 (Intel)
- ✅ macOS ARM64 (Apple Silicon)
- ✅ Windows x86_64 (MSVC)
- ✅ Windows ARM64 (MSVC)

### 2. `ci.yml` - 持续集成工作流

**触发条件**:

- 推送到 `main` 或 `develop` 分支
- 提交 Pull Request 到 `main` 或 `develop`

**功能**:

- 代码格式检查 (rustfmt)
- Lint 检查 (clippy)
- 单元测试
- 多平台编译验证

---

## 🚀 快速开始

### 第一步: 初始化 Git 仓库

如果还未初始化：

```bash
cd /Users/oort/Documents/project/ai-code/CoinAddress

# 初始化 Git
git init

# 添加远程仓库 (替换为你的仓库)
git remote add origin https://github.com/YOUR_USERNAME/CoinAddress.git

# 提交所有文件
git add .
git commit -m "Initial commit: TRON vanity address generator with cross-platform build support"

# 推送到 main 分支
git branch -M main
git push -u origin main
```

### 第二步: 创建 Release

```bash
# 创建 tag
git tag -a v0.2.0 -m "Release version 0.2.0: Windows ARM64 support"

# 推送 tag (触发 Release 工作流)
git push origin v0.2.0
```

GitHub 会自动:

1. 检测新 tag
2. 创建 GitHub Release
3. 启动 6 个编译任务 (平行执行)
4. 上传编译结果到 Release 页面

### 第三步: 下载二进制文件

访问 GitHub Release 页面:

```
https://github.com/YOUR_USERNAME/CoinAddress/releases/tag/v0.2.0
```

下载对应平台的文件:

- `tron-vanity_0.2.0_linux-x86_64.tar.gz` - Linux 64 位
- `tron-vanity_0.2.0_linux-arm64.tar.gz` - Linux ARM64
- `tron-vanity_0.2.0_macos-x86_64.tar.gz` - macOS Intel
- `tron-vanity_0.2.0_macos-arm64.tar.gz` - macOS Apple Silicon
- `tron-vanity_0.2.0_windows-x86_64.zip` - Windows 64 位
- `tron-vanity_0.2.0_windows-arm64.zip` - Windows ARM64

---

## 📦 输出文件格式

### Linux/macOS (TAR.GZ)

```
tron-vanity_0.2.0_linux-x86_64.tar.gz
└── tron-vanity          (可执行文件)
```

使用:

```bash
tar -xzf tron-vanity_0.2.0_linux-x86_64.tar.gz
./tron-vanity
```

### Windows (ZIP)

```
tron-vanity_0.2.0_windows-x86_64.zip
├── tron-vanity.exe      (可执行文件)
├── run.bat              (带暂停)
└── run-silent.bat       (后台运行)
```

使用:

- 双击 `run.bat` - 运行后显示输出，按任意键退出
- 双击 `run-silent.bat` - 后台运行
- 或在命令行: `tron-vanity.exe`

---

## 🔧 配置说明

### 环境变量

所有工作流都配置了:

```yaml
CARGO_TERM_COLOR: always # 彩色输出
RUST_BACKTRACE: 1 # 错误堆栈跟踪
```

### 缓存

CI 工作流启用了 Cargo 缓存，加快后续编译:

```yaml
key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### 代码签名

macOS 版本自动应用 Ad-hoc 代码签名:

```yaml
codesign -s - target/x86_64-apple-darwin/release/tron-vanity || true
```

---

## 📊 工作流执行流程

### Release 工作流

```
Tag created (v0.2.0)
    ↓
create-release (创建 GitHub Release)
    ↓
    ├─→ build-linux-x86_64 ─┐
    ├─→ build-linux-arm64   ├─→ Upload to Release
    ├─→ build-macos-x86_64  │
    ├─→ build-macos-arm64   ├─→ (并行执行，时间 ~5-15 分钟)
    ├─→ build-windows-x86_64│
    └─→ build-windows-arm64 ┘
    ↓
publish-complete (完成通知)
```

### CI 工作流

```
Push to main/develop 或 PR created
    ↓
    ├─→ check (cargo check)
    ├─→ fmt (rustfmt 检查)
    ├─→ clippy (Lint 检查)
    ├─→ test (运行测试)
    └─→ build (多平台编译验证)
    ↓
All jobs completed
```

---

## 🐛 故障排查

### 问题: 编译失败

**解决方案**:

1. 查看工作流日志: GitHub Actions 标签 → 点击失败的工作流
2. 检查错误信息
3. 常见原因:
   - 缺少依赖库
   - Rust 版本不兼容
   - 源码有编译错误

### 问题: Release 不创建

**解决方案**:

1. 确认 tag 格式: `v*` (例如 `v0.2.0`)
2. 检查是否推送了 tag: `git push origin v0.2.0`
3. 查看 GitHub Actions 日志

### 问题: 文件未上传到 Release

**解决方案**:

1. 检查 upload_url 是否正确
2. 确认编译成功 (查看日志)
3. 检查文件权限

---

## 💡 常用命令

### 创建 Release

```bash
# 创建本地 tag
git tag -a v0.2.1 -m "Release 0.2.1"

# 推送 tag
git push origin v0.2.1

# 或一步完成
git tag -a v0.2.1 -m "Release 0.2.1" && git push origin v0.2.1
```

### 查看 tag

```bash
# 列出所有 tag
git tag

# 查看特定 tag
git show v0.2.1

# 删除本地 tag
git tag -d v0.2.1

# 删除远程 tag
git push origin --delete v0.2.1
```

### 推送提交到触发 CI

```bash
# 推送提交
git push origin main

# 提交自动触发 CI 工作流
```

---

## 📈 性能优化

### 编译时间

当前配置:

- **第一次编译**: ~5-15 分钟 (取决于平台)
- **增量编译**: ~2-5 分钟 (缓存命中)

### 并行构建

所有平台并行编译，总时间约 = 最慢的单个任务 (~15 分钟)

---

## 🔐 安全性

### GitHub Token

工作流使用 `secrets.GITHUB_TOKEN` 进行认证:

- 自动提供，无需手动配置
- 权限限制为当前 repo
- 每次工作流运行时自动轮换

### 代码签名

macOS 使用 Ad-hoc 代码签名 (开发用):

```yaml
codesign -s - target/x86_64-apple-darwin/release/tron-vanity || true
```

生产环境可配置证书签名 (需要 Apple 开发者账户)

---

## 📝 版本管理

### 推荐的版本命名

使用语义化版本 (SemVer):

- `v0.2.0` - 主版本.次版本.补丁版本
- `v0.2.1-beta.1` - 带后缀标签
- `v1.0.0-rc.1` - 发布候选版

### Tag 约定

- 正式版本: `v1.0.0`
- 测试版本: `v1.0.0-beta.1`
- 每个 tag 都应有对应的 Release 说明

---

## 📚 相关文件

- `.github/workflows/release.yml` - Release 发布工作流
- `.github/workflows/ci.yml` - 持续集成工作流
- `Cargo.toml` - Rust 项目配置
- `.cargo/config.toml` - 平台特定编译配置

---

## 🎯 下一步

1. ✅ **初始化 Git 仓库** → 添加远程仓库
2. ✅ **提交代码** → `git push origin main`
3. ✅ **创建 Release** → `git tag -a v0.2.0 -m "..."`
4. ✅ **推送 tag** → `git push origin v0.2.0`
5. ✅ **监控编译** → 访问 GitHub Actions 标签
6. ✅ **发布完成** → 从 Release 页面下载二进制文件

---

**更新时间**: 2024-12-07  
**状态**: ✅ 完成  
**支持平台**: Linux x86_64/ARM64, macOS Intel/Apple Silicon, Windows x86_64/ARM64

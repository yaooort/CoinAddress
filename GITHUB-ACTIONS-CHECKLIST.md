# GitHub Actions 部署检查清单

按照此清单完成 GitHub Actions 的设置和部署。

## ✅ 环境准备

- [ ] 已安装 Git (`git --version`)
- [ ] 已配置 Git 用户名和邮箱
  ```bash
  git config --global user.name "Your Name"
  git config --global user.email "your.email@example.com"
  ```
- [ ] 有 GitHub 账户
- [ ] 在 GitHub 上创建了新的空仓库

## ✅ 本地 Git 仓库设置

- [ ] 项目已包含以下工作流文件:

  - [ ] `.github/workflows/release.yml` (9.5 KB)
  - [ ] `.github/workflows/ci.yml` (3.4 KB)

- [ ] 项目包含配置文件:

  - [ ] `Cargo.toml` (项目配置)
  - [ ] `.cargo/config.toml` (平台编译配置)
  - [ ] `Cargo.lock` (依赖锁定)

- [ ] 项目根目录包含:
  - [ ] `src/` (源代码目录)
  - [ ] `Cargo.toml`
  - [ ] `README.md`

## ✅ Git 初始化

```bash
cd /Users/oort/Documents/project/ai-code/CoinAddress

# 初始化 Git (如果还未初始化)
git init

# 添加远程仓库 (替换为你的仓库 URL)
git remote add origin https://github.com/YOUR_USERNAME/CoinAddress.git

# 检查 remote
git remote -v
```

- [ ] Git 仓库已初始化
- [ ] 远程仓库地址正确
- [ ] 可以连接到 GitHub (测试: `git remote -v`)

## ✅ 代码提交

```bash
# 查看状态
git status

# 添加所有文件
git add .

# 提交
git commit -m "Initial commit: TRON vanity address generator with GitHub Actions"

# 设置默认分支为 main
git branch -M main

# 推送到 GitHub
git push -u origin main
```

- [ ] 所有文件已添加到 Git
- [ ] 初始提交已完成
- [ ] 代码已推送到 GitHub main 分支
- [ ] 可以在 GitHub 网页上看到代码

## ✅ 工作流验证

访问 GitHub 仓库:

```
https://github.com/YOUR_USERNAME/CoinAddress
```

- [ ] 点击 "Actions" 标签
- [ ] 验证 CI 工作流是否已运行
- [ ] 检查 CI 状态:
  - [ ] ✓ check - 代码检查通过
  - [ ] ✓ fmt - 代码格式正确
  - [ ] ✓ clippy - Lint 检查通过
  - [ ] ✓ test - 单元测试通过
  - [ ] ✓ build - 多平台编译成功

> 注: 首次推送后，CI 工作流会自动运行

## ✅ 创建 Release

### 方式 1: 命令行创建 Tag

```bash
# 创建 tag (版本号格式: v主.次.补丁)
git tag -a v0.2.0 -m "Release 0.2.0: Windows cross-compilation support"

# 推送 tag 到 GitHub (触发 Release 工作流)
git push origin v0.2.0
```

### 方式 2: GitHub 网页创建 Release

1. 访问仓库 → Releases 标签
2. 点击 "Create a new release"
3. 输入版本号 (v0.2.0)
4. 发布

选择其中一种方式:

- [ ] 已创建 tag 并推送
- [ ] 或在 GitHub 网页上创建了 Release

## ✅ 监控 Release 编译

访问: `https://github.com/YOUR_USERNAME/CoinAddress/actions`

- [ ] Release 工作流已启动
- [ ] 可以看到 6 个编译任务:
  - [ ] build-linux-x86_64
  - [ ] build-linux-arm64
  - [ ] build-macos-x86_64
  - [ ] build-macos-arm64
  - [ ] build-windows-x86_64
  - [ ] build-windows-arm64

任务状态:

- [ ] ⏳ 进行中 (正在编译)
- [ ] ✓ 成功 (编译完成)
- [ ] ✗ 失败 (有错误)

## ✅ 编译完成

访问 Release 页面: `https://github.com/YOUR_USERNAME/CoinAddress/releases/tag/v0.2.0`

验证已上传的文件:

- [ ] tron-vanity_0.2.0_linux-x86_64.tar.gz
- [ ] tron-vanity_0.2.0_linux-arm64.tar.gz
- [ ] tron-vanity_0.2.0_macos-x86_64.tar.gz
- [ ] tron-vanity_0.2.0_macos-arm64.tar.gz
- [ ] tron-vanity_0.2.0_windows-x86_64.zip
- [ ] tron-vanity_0.2.0_windows-arm64.zip

所有文件大小:

- [ ] 每个文件大小 > 5 MB (表示编译成功)
- [ ] 文件类型正确 (.tar.gz 或 .zip)

## ✅ 测试下载

- [ ] 下载一个 Release 文件
- [ ] 解压文件
- [ ] 验证二进制文件存在
- [ ] 尝试运行二进制文件

### Linux/macOS 测试

```bash
# 下载
wget https://github.com/YOUR_USERNAME/CoinAddress/releases/download/v0.2.0/tron-vanity_0.2.0_linux-x86_64.tar.gz

# 解压
tar -xzf tron-vanity_0.2.0_linux-x86_64.tar.gz

# 运行
./tron-vanity

# 显示帮助
./tron-vanity --help
```

### Windows 测试

1. 下载 `tron-vanity_0.2.0_windows-x86_64.zip`
2. 解压到文件夹
3. 双击 `run.bat` 或 `run-silent.bat`
4. 验证程序运行

## ✅ 持续集成 - 后续提交

之后每次提交都会自动运行 CI:

```bash
# 修改代码
echo "// New feature" >> src/main.rs

# 提交
git add .
git commit -m "Add new feature"

# 推送 (自动触发 CI)
git push origin main
```

- [ ] 理解 CI 工作流会自动运行
- [ ] 知道如何在 GitHub Actions 标签查看状态
- [ ] 明白代码必须通过所有检查才能合并

## ✅ 后续更新 - 创建下一个 Release

```bash
# 修改版本号 (Cargo.toml 中的 version)
# 然后:

git tag -a v0.2.1 -m "Release 0.2.1: Bug fixes"
git push origin v0.2.1
```

- [ ] 了解如何创建新的 Release
- [ ] 知道版本号格式 (v 主.次.补丁)

## 📝 常见问题

### Q: 编译失败怎么办?

A: 查看 GitHub Actions 日志:

1. 访问 Actions 标签
2. 点击失败的工作流
3. 查看错误信息
4. 检查源代码是否有编译错误

### Q: 如何修改工作流?

A: 编辑 `.github/workflows/` 中的 YAML 文件并推送

### Q: 能否只编译某个平台?

A: 可以，修改 `release.yml` 中的 `needs` 字段

### Q: Release 如何更新?

A: 创建新的 tag，会自动创建新的 Release

### Q: 如何删除错误的 Release?

A: GitHub 网页上 Release 页面点击删除，或:

```bash
git tag -d v0.2.0
git push origin --delete v0.2.0
```

## ✅ 最终检查

- [ ] 所有 CI 检查通过 (绿色 ✓)
- [ ] 所有 Release 文件已上传
- [ ] 能够成功下载和运行二进制文件
- [ ] 理解整个 GitHub Actions 工作流
- [ ] 知道如何创建新的 Release

## 🎉 完成!

恭喜！你已经成功设置了完整的自动化编译和发布系统。

现在:

1. 提交代码更改
2. 创建 Release tag
3. GitHub Actions 自动编译所有平台
4. 用户从 Release 页面下载

无需手动参与！

---

**参考文档**:

- `GITHUB-ACTIONS-GUIDE.md` - 完整指南
- `.github/workflows/release.yml` - Release 工作流
- `.github/workflows/ci.yml` - CI 工作流

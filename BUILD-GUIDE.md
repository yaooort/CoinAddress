# 📦 TRON Vanity 交叉编译与 Logo 指南

## 概述

本项目提供了完整的交叉编译工具链和品牌资源生成工具，支持以下平台：

### 支持的平台

| 平台        | x86_64 | ARM64/aarch64      |
| ----------- | ------ | ------------------ |
| **Linux**   | ✅     | ✅                 |
| **Windows** | ✅     | ✅                 |
| **macOS**   | ✅     | ✅ (Apple Silicon) |

---

## 🚀 快速开始

### 最快的方式：一键编译所有平台

```bash
# 需要在macOS上才能同时编译macOS版本
./quick-build.sh
```

这个脚本会：

1. ✅ 检查所有必要工具
2. 📦 自动添加编译目标
3. 🔨 依次编译所有平台版本
4. 📁 输出到 `dist/` 目录

**输出示例：**

```
dist/
├── linux-0.2.0           (Linux x86_64)
├── linux-arm-0.2.0       (Linux ARM64)
├── windows-0.2.0.exe     (Windows x86_64)
├── windows-arm-0.2.0.exe (Windows ARM64)
├── macos-0.2.0           (macOS x86_64)
└── macos-arm-0.2.0       (macOS ARM64)
```

---

## 🛠️ 高级编译选项

### 完整的交叉编译脚本

```bash
./build-cross.sh [选项]
```

#### 可用选项

| 选项            | 说明                  |
| --------------- | --------------------- |
| `--build-all`   | 编译所有平台（默认）  |
| `--linux-x64`   | 仅编译 Linux x86_64   |
| `--linux-arm`   | 仅编译 Linux ARM64    |
| `--win-x64`     | 仅编译 Windows x86_64 |
| `--win-arm`     | 仅编译 Windows ARM64  |
| `--macos-x64`   | 仅编译 macOS x86_64   |
| `--macos-arm`   | 仅编译 macOS ARM64    |
| `--add-targets` | 仅添加编译目标        |
| `--deps`        | 仅检查依赖            |
| `--clean`       | 清理构建文件          |
| `--help`        | 显示帮助信息          |

#### 使用示例

```bash
# 仅编译Linux版本
./build-cross.sh --linux-x64

# 编译Windows和macOS
./build-cross.sh --win-x64 --macos-x64

# 清理所有构建
./build-cross.sh --clean
```

---

## 🎨 Logo 和品牌资源

### 生成所有 Logo

```bash
python3 generate-logo.py
```

这会生成以下文件到 `assets/logos/` 目录：

| 文件          | 尺寸     | 用途                    |
| ------------- | -------- | ----------------------- |
| `logo.svg`    | 512x512  | 主 Logo、网站、社交媒体 |
| `icon.svg`    | 256x256  | 应用图标、桌面快捷方式  |
| `banner.svg`  | 1200x400 | GitHub README、营销资料 |
| `favicon.svg` | 64x64    | 网站标签栏、浏览器书签  |

### 颜色方案

```
🟠 主色 (Orange):  #ff6b35 RGB(255, 107, 53)
🔵 副色 (Cyan):    #40d4ff RGB(64, 212, 255)
🟡 强调 (Gold):    #f7931a RGB(247, 147, 26)
```

### 预览

所有 Logo 都包含以下设计元素：

- ✨ 闪光动画效果
- 🔄 旋转环形动画
- 💎 几何菱形和星形
- 🎨 渐变色彩方案

---

## 📋 构建流程详解

### 1. 前置条件

```bash
# 需要安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 2. 添加编译目标

```bash
# 自动添加（脚本会执行）
./build-cross.sh --add-targets

# 或手动添加
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnu
rustup target add x86_64-apple-darwin      # macOS only
rustup target add aarch64-apple-darwin     # macOS only
```

### 3. 编译

```bash
# 编译特定目标
cargo build --release --target x86_64-unknown-linux-gnu

# 编译输出
# target/x86_64-unknown-linux-gnu/release/tron-vanity
```

### 4. 打包和分发

脚本会自动：

- 💾 复制二进制文件到 `dist/` 目录
- 📄 包含 README 和文档
- 🐚 生成启动脚本（Unix-like 系统）
- 📦 创建 ZIP 或 TAR.GZ 压缩包

---

## 🔧 平台特定说明

### Linux

编译要求：

- GCC 工具链
- 标准开发工具（build-essential）

```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/CentOS
sudo yum groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

### Windows

编译要求：

- MinGW-w64 工具链
- 或使用 Visual C++ Build Tools

```bash
# 通过Rust安装（推荐）
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnu
```

### macOS

编译要求：

- Xcode Command Line Tools
- clang 编译器

```bash
# 安装Xcode工具
xcode-select --install

# 验证
clang --version
```

---

## 📊 编译性能优化

### Release Profile 配置

项目已配置最优化设置：

```toml
[profile.release]
opt-level = 3           # 最高优化级别
lto = true              # 链接时优化
codegen-units = 1       # 最小代码生成单元
strip = true            # 剥离调试符号
```

### 编译时间

| 目标           | 首次编译  | 增量编译 |
| -------------- | --------- | -------- |
| Linux x86_64   | ~2 分钟   | 10 秒    |
| Linux ARM64    | ~2 分钟   | 10 秒    |
| Windows x86_64 | ~2 分钟   | 10 秒    |
| Windows ARM64  | ~2 分钟   | 10 秒    |
| macOS x86_64   | ~1.5 分钟 | 8 秒     |
| macOS ARM64    | ~1.5 分钟 | 8 秒     |

---

## 🐛 故障排查

### 问题：未找到编译工具

```bash
# 解决方案：添加目标
rustup target add <target>
```

### 问题：链接错误

```bash
# 解决方案：检查交叉编译工具是否安装
# Linux: sudo apt-get install mingw-w64
# macOS: xcode-select --install
```

### 问题：权限不足

```bash
# 解决方案：给脚本添加执行权限
chmod +x *.sh
```

---

## 📦 分发建议

### 目录结构

```
tron-vanity-0.2.0/
├── README.md              # 使用说明
├── PACKAGING.md           # 打包说明
├── tron-vanity            # 可执行文件（Linux/macOS）
├── tron-vanity.exe        # 可执行文件（Windows）
├── run.sh                 # 启动脚本（Unix）
└── run.bat                # 启动脚本（Windows）
```

### 上传到 GitHub Releases

```bash
# 1. 编译所有版本
./quick-build.sh

# 2. 创建发布标签
git tag v0.2.0
git push origin v0.2.0

# 3. 上传到Releases
# 使用GitHub Web界面或gh命令
gh release create v0.2.0 dist/*
```

---

## 📝 脚本说明

### build-cross.sh

- **目的**: 完整的交叉编译工具
- **功能**: 支持单个或批量编译，自动打包
- **用途**: 专业发布和分发

### quick-build.sh

- **目的**: 快速一键编译
- **功能**: 智能检测，自动添加目标，批量编译
- **用途**: 日常开发和快速构建

### deploy-resources.sh

- **目的**: 部署应用资源
- **功能**: 配置平台特定的资源和快捷方式
- **用途**: 应用打包和部署

### generate-logo.py

- **目的**: 生成品牌 Logo
- **功能**: 创建多种格式的 Logo 和图标
- **用途**: 品牌推广和应用包装

---

## 🎯 最佳实践

1. **开发环节**: 使用 `quick-build.sh` 快速验证
2. **发布环节**: 使用 `build-cross.sh --build-all` 完整编译
3. **测试环节**: 在每个平台上实际运行测试版本
4. **分发环节**: 上传到 GitHub Releases 或其他分发渠道

---

## ❓ 常见问题

**Q: 如何在 Windows 上交叉编译 Linux 版本？**

```bash
# 使用专业工具链
./build-cross.sh --linux-x64
```

**Q: 编译时间太长怎么办？**

```bash
# 使用增量编译或特定目标
cargo build --release --target x86_64-linux-gnu
```

**Q: 能否修改 Logo 颜色？**

```bash
# 编辑 generate-logo.py 中的颜色值
# 然后重新运行
python3 generate-logo.py
```

---

## 📞 技术支持

遇到问题？

1. 查看 `PACKAGING.md` 获取打包说明
2. 查看 `README.md` 获取使用说明
3. 运行 `./build-cross.sh --help` 获取脚本帮助

---

**最后更新**: 2024-12-07
**版本**: 0.2.0
**维护者**: TRON Vanity Team

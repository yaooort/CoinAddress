# 🚀 TRON Vanity - 完整部署指南

## 📋 目录

1. [快速开始](#快速开始)
2. [交叉编译](#交叉编译)
3. [构建所有平台](#构建所有平台)
4. [Logo 和品牌](#logo-和品牌)
5. [打包和分发](#打包和分发)
6. [CI/CD 集成](#cicd-集成)
7. [故障排查](#故障排查)

---

## 🎯 快速开始

### 最快的方式

```bash
# 编译所有平台（需要在macOS上）
./quick-build.sh

# 输出到 dist/ 目录
ls -lh dist/
```

### 编译特定平台

```bash
# 仅编译Linux x86_64
./build-cross.sh --linux-x64

# 仅编译Windows
./build-cross.sh --win-x64 --win-arm

# 仅编译macOS
./build-cross.sh --macos-x64 --macos-arm
```

---

## 🔨 交叉编译

### 支持的平台

```
✅ Linux:     x86_64, ARM64
✅ Windows:   x86_64, ARM64
✅ macOS:     x86_64, ARM64 (Apple Silicon)
```

### 前置要求

#### 通用要求

```bash
# 安装Rust工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 验证安装
rustc --version && cargo --version
```

#### 平台特定要求

**Linux (Ubuntu/Debian):**

```bash
sudo apt-get update
sudo apt-get install build-essential mingw-w64
```

**macOS:**

```bash
xcode-select --install
```

**Windows (使用 WSL2 或 MSYS2):**

```bash
# 已通过rustup包含
rustup target add x86_64-pc-windows-gnu
```

### 编译步骤

#### 1. 添加编译目标

```bash
./build-cross.sh --add-targets
```

或手动添加：

```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnu
rustup target add x86_64-apple-darwin      # macOS only
rustup target add aarch64-apple-darwin     # macOS only
```

#### 2. 编译

```bash
# 编译单个目标
cargo build --release --target x86_64-unknown-linux-gnu

# 或使用脚本编译所有
./build-cross.sh --build-all
```

#### 3. 验证

```bash
# 检查二进制文件
ls -lh dist/
file dist/linux-0.2.0      # 应显示 ELF 64-bit
file dist/windows-0.2.0.exe  # 应显示 PE 32-bit
```

---

## 📦 构建所有平台

### 一条命令搞定

```bash
./quick-build.sh
```

### 详细流程

```bash
# 1. 检查环境
./build-cross.sh --deps

# 2. 添加目标
./build-cross.sh --add-targets

# 3. 编译所有平台
./build-cross.sh --build-all

# 4. 查看结果
ls -lh dist/
```

### 输出结构

```
dist/
├── linux-0.2.0              # Linux x86_64 (ELF binary)
├── linux-arm-0.2.0          # Linux ARM64 (ELF binary)
├── windows-0.2.0.exe        # Windows x86_64 (PE binary)
├── windows-arm-0.2.0.exe    # Windows ARM64 (PE binary)
├── macos-0.2.0              # macOS x86_64 (Mach-O binary)
└── macos-arm-0.2.0          # macOS ARM64 (Mach-O binary)
```

---

## 🎨 Logo 和品牌

### 生成 Logo

```bash
python3 generate-logo.py
```

### 生成的文件

```
assets/logos/
├── logo.svg           # 主Logo (512x512)
├── icon.svg          # 应用图标 (256x256)
├── banner.svg        # 横幅 (1200x400)
├── favicon.svg       # Favicon (64x64)
└── preview.html      # 预览页面
```

### 查看预览

```bash
# 在浏览器中打开
open assets/logos/preview.html
```

或使用 Python 简单服务器：

```bash
cd assets/logos
python3 -m http.server 8000
# 打开 http://localhost:8000/preview.html
```

### 颜色方案

```
🟠 主色 (Orange):  #ff6b35  RGB(255, 107, 53)
🔵 副色 (Cyan):    #40d4ff  RGB(64, 212, 255)
🟡 强调 (Gold):    #f7931a  RGB(247, 147, 26)
```

### 自定义 Logo

编辑 `generate-logo.py` 中的颜色值，然后重新运行：

```python
# 修改这些颜色
stop-color:#ff6b35  # 改为你想要的颜色
stop-color:#40d4ff
stop-color:#f7931a
```

---

## 📦 打包和分发

### 创建发行版本

#### 1. 编译所有平台

```bash
./quick-build.sh
```

#### 2. 创建版本标签

```bash
git tag v0.2.0
git push origin v0.2.0
```

#### 3. 创建 GitHub Release

```bash
# 方法 1: 使用 gh 命令行工具
gh release create v0.2.0 \
  dist/linux-0.2.0 \
  dist/linux-arm-0.2.0 \
  dist/windows-0.2.0.exe \
  dist/windows-arm-0.2.0.exe \
  dist/macos-0.2.0 \
  dist/macos-arm-0.2.0 \
  --title "TRON Vanity v0.2.0" \
  --notes "Release notes here"
```

```bash
# 方法 2: 使用 GitHub Web 界面
# 1. 访问 https://github.com/username/repo/releases
# 2. 点击 "Create a new release"
# 3. 上传文件
```

### 打包为存档

```bash
cd dist

# Linux 和 macOS
tar -czf tron-vanity-0.2.0-linux-x86_64.tar.gz linux-0.2.0
tar -czf tron-vanity-0.2.0-linux-arm64.tar.gz linux-arm-0.2.0
tar -czf tron-vanity-0.2.0-macos-x86_64.tar.gz macos-0.2.0
tar -czf tron-vanity-0.2.0-macos-arm64.tar.gz macos-arm-0.2.0

# Windows (使用PowerShell)
Compress-Archive -Path windows-0.2.0.exe -DestinationPath tron-vanity-0.2.0-windows-x86_64.zip
Compress-Archive -Path windows-arm-0.2.0.exe -DestinationPath tron-vanity-0.2.0-windows-arm64.zip
```

### 生成校验和

```bash
cd dist

# Linux/macOS
sha256sum * > SHA256SUMS
md5sum * > MD5SUMS

# Windows (PowerShell)
Get-FileHash * -Algorithm SHA256 | Format-Table Path, Hash > SHA256SUMS.txt
```

---

## 🔄 CI/CD 集成

### GitHub Actions 工作流

创建 `.github/workflows/build.yml`:

```yaml
name: Cross-Compile Build

on:
  push:
    tags:
      - "v*"

jobs:
  build:
    name: Build
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            name: linux-x86_64
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            name: linux-arm64
          - os: windows-latest
            target: x86_64-pc-windows-gnu
            name: windows-x86_64
          - os: windows-latest
            target: aarch64-pc-windows-gnu
            name: windows-arm64
          - os: macos-latest
            target: x86_64-apple-darwin
            name: macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            name: macos-arm64

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.name }}
          path: target/${{ matrix.target }}/release/tron-vanity*

  release:
    name: Create Release
    runs-on: ubuntu-latest
    needs: build
    if: startsWith(github.ref, 'refs/tags/')

    steps:
      - uses: actions/checkout@v3

      - name: Download all artifacts
        uses: actions/download-artifact@v3

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            linux-x86_64/*
            linux-arm64/*
            windows-x86_64/*
            windows-arm64/*
            macos-x86_64/*
            macos-arm64/*
```

### 本地 CI 测试

```bash
# 模拟CI环境测试
./build-cross.sh --linux-x64
./build-cross.sh --win-x64
./build-cross.sh --macos-x64
```

---

## 🐛 故障排查

### 问题: 编译错误 "undefined reference"

**原因**: 交叉编译工具未正确安装

**解决**:

```bash
# Linux
sudo apt-get install mingw-w64

# 重新添加目标
rustup target add x86_64-pc-windows-gnu

# 清理并重建
cargo clean
cargo build --release --target x86_64-pc-windows-gnu
```

### 问题: "target triple not installed"

**解决**:

```bash
# 查看已安装的目标
rustup target list | grep installed

# 添加缺失的目标
rustup target add <target>
```

### 问题: Windows exe 无法运行

**原因**: 可能缺少运行库

**解决**:

```bash
# 在Windows上测试运行
.\dist\windows-0.2.0.exe

# 如果提示缺少DLL，安装VC运行库
# https://support.microsoft.com/en-us/help/2977003
```

### 问题: macOS 应用无法打开

**原因**: 权限或代码签名问题

**解决**:

```bash
# 检查权限
ls -lh dist/macos-0.2.0

# 添加可执行权限
chmod +x dist/macos-0.2.0

# 测试运行
./dist/macos-0.2.0
```

### 问题: 编译太慢

**优化**:

```bash
# 使用增量编译
touch Cargo.toml
cargo build --release --target x86_64-linux-gnu

# 或仅编译需要的目标
./build-cross.sh --linux-x64
```

---

## 📊 性能指标

### 编译时间

| 目标           | 首次编译 | 增量编译 |
| -------------- | -------- | -------- |
| Linux x86_64   | ~90s     | ~10s     |
| Linux ARM64    | ~90s     | ~10s     |
| Windows x86_64 | ~90s     | ~10s     |
| Windows ARM64  | ~90s     | ~10s     |
| macOS x86_64   | ~70s     | ~8s      |
| macOS ARM64    | ~70s     | ~8s      |

### 二进制大小

| 目标           | 大小   |
| -------------- | ------ |
| Linux x86_64   | ~15 MB |
| Linux ARM64    | ~15 MB |
| Windows x86_64 | ~16 MB |
| Windows ARM64  | ~16 MB |
| macOS x86_64   | ~14 MB |
| macOS ARM64    | ~14 MB |

---

## 📚 相关文档

- [README.md](README.md) - 项目说明
- [BUILD-GUIDE.md](BUILD-GUIDE.md) - 详细构建指南
- [PACKAGING.md](PACKAGING.md) - 打包说明

---

## 💡 最佳实践

1. **开发**: 在主平台编译和测试
2. **预发布**: 编译所有目标验证
3. **发布**: 生成校验和和更新日志
4. **维护**: 保持依赖项最新

---

## 📞 支持

遇到问题？

1. 查看 `TROUBLESHOOTING.md`
2. 运行 `./build-cross.sh --help`
3. 检查 GitHub Issues

---

**最后更新**: 2024-12-07  
**版本**: 0.2.0

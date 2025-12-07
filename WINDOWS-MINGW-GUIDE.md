# 🪟 Windows MinGW 交叉编译指南

## 概述

本指南介绍如何在 Linux、macOS 或 Windows 上使用 MinGW-w64 工具链交叉编译 Windows 版本的 TRON Vanity。

## 支持的组合

| 构建系统    | 目标系统       | 编译工具      | 状态 |
| ----------- | -------------- | ------------- | ---- |
| **macOS**   | Windows x86_64 | MinGW-w64     | ✅   |
| **macOS**   | Windows ARM64  | MinGW-w64     | ✅   |
| **Linux**   | Windows x86_64 | MinGW-w64     | ✅   |
| **Linux**   | Windows ARM64  | MinGW-w64     | ✅   |
| **Windows** | Windows x86_64 | MSVC 或 MinGW | ✅   |
| **Windows** | Windows ARM64  | MSVC 或 MinGW | ✅   |

---

## 前置要求

### macOS

使用 Homebrew 安装 MinGW-w64：

```bash
brew install mingw-w64
```

验证安装：

```bash
x86_64-w64-mingw32-gcc --version
aarch64-w64-mingw32-gcc --version
```

### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install mingw-w64
```

验证安装：

```bash
x86_64-w64-mingw32-gcc --version
aarch64-w64-mingw32-gcc --version
```

### Linux (Fedora/CentOS/RHEL)

```bash
sudo dnf install mingw-w64-gcc mingw-w64-gcc-c++
```

### Linux (Arch)

```bash
sudo pacman -S mingw-w64
```

### Windows

在 Windows 上，可以使用：

1. **MSVC** (推荐):

   ```bash
   cargo build --release --target x86_64-pc-windows-msvc
   ```

2. **MinGW** (通过 MSYS2):
   ```bash
   pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-g++
   ```

---

## 安装 Rust 编译目标

添加 Windows 目标到 Rust：

```bash
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnu
```

列出已安装的目标：

```bash
rustup target list | grep windows
```

---

## 快速编译

### 方式 1: 使用一键脚本 (推荐)

```bash
chmod +x build-windows.sh
./build-windows.sh
```

这将：

1. 检查 MinGW 工具
2. 编译 Windows x86_64 版本
3. 编译 Windows ARM64 版本
4. 打包为 ZIP 文件

### 方式 2: 手动编译

#### 编译 Windows x86_64

```bash
export CC=x86_64-w64-mingw32-gcc
export CXX=x86_64-w64-mingw32-g++
cargo build --release --target x86_64-pc-windows-gnu
```

输出文件：`target/x86_64-pc-windows-gnu/release/tron-vanity.exe`

#### 编译 Windows ARM64

```bash
export CC=aarch64-w64-mingw32-gcc
export CXX=aarch64-w64-mingw32-g++
cargo build --release --target aarch64-pc-windows-gnu
```

输出文件：`target/aarch64-pc-windows-gnu/release/tron-vanity.exe`

### 方式 3: 使用 Cargo 配置

`.cargo/config.toml` 已预配置了 MinGW 工具链。直接运行：

```bash
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target aarch64-pc-windows-gnu
```

---

## 高级配置

### 自定义编译器

编辑 `.cargo/config.toml` 中的 `[target.x86_64-pc-windows-gnu]` 部分：

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"
rustflags = [
    "-C", "link-arg=-static-libgcc",
    "-C", "opt-level=3",
]
```

### 环境变量

也可以通过环境变量设置：

```bash
export CC=x86_64-w64-mingw32-gcc
export CXX=x86_64-w64-mingw32-g++
export AR=x86_64-w64-mingw32-ar
export RUSTFLAGS="-C link-arg=-static-libgcc"
```

### 链接选项

**静态链接 GCC 运行库**（推荐）：

```bash
rustflags = ["-C", "link-arg=-static-libgcc"]
```

**动态链接**（需要在 Windows 上安装 GCC 运行库）：

```bash
# 不设置 -static-libgcc
```

---

## 编译错误排查

### 错误：找不到 MinGW 工具

```bash
# 检查是否安装
which x86_64-w64-mingw32-gcc

# 或
x86_64-w64-mingw32-gcc --version
```

**解决方案**：

- macOS: `brew install mingw-w64`
- Linux: `sudo apt-get install mingw-w64`

### 错误：undefined reference 到 Windows API

这通常是因为链接了错误的库。检查：

```bash
# 查看链接依赖
x86_64-w64-mingw32-gcc -print-search-dirs

# 查看编译的完整命令
RUSTFLAGS="-Z print-link-args" cargo build --target x86_64-pc-windows-gnu
```

### 错误：无法找到目标三元组

```bash
# 添加缺失的目标
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnu
```

### 编译很慢

使用增量编译或只编译需要的部分：

```bash
# 增量编译（修改代码后）
touch Cargo.toml
cargo build --release --target x86_64-pc-windows-gnu

# 仅编译库
cargo build --release --target x86_64-pc-windows-gnu --lib
```

---

## 在 Windows 上运行编译结果

### 测试编译的二进制

在 Windows 机器上：

```bash
# x86_64 版本
tron-vanity_0.2.0_windows-x86_64.zip
# 解压后双击 run.bat 或运行 tron-vanity.exe

# ARM64 版本 (Windows 11 ARM64 Edition)
tron-vanity_0.2.0_windows-arm64.zip
# 解压后双击 run.bat 或运行 tron-vanity.exe
```

### 依赖要求

对于 ARM64 编译的二进制，需要：

- **Windows 11 或更高版本**（支持 ARM64）
- **ARM64 处理器**（如高通骁龙 8cx）

---

## 性能优化

### 编译优化

项目已配置了最优化的 Release Profile：

```toml
[profile.release]
opt-level = 3           # 最高优化
lto = true              # 链接时优化
codegen-units = 1       # 最小代码单元
strip = true            # 剥离调试符号
panic = "abort"         # 快速 panic
```

### 二进制大小

编译后的二进制大小（Strip 后）：

| 目标   | 大小      |
| ------ | --------- |
| x86_64 | ~14-16 MB |
| ARM64  | ~14-16 MB |

### 编译时间

| 情况         | 时间    |
| ------------ | ------- |
| 首次编译     | ~90 秒  |
| 增量编译     | ~10 秒  |
| 全量重新编译 | ~2 分钟 |

---

## 集成 CI/CD

### GitHub Actions 示例

在 `.github/workflows/build-windows.yml` 中：

```yaml
name: Build Windows

on:
  push:
    tags:
      - "v*"

jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-pc-windows-gnu
          - aarch64-pc-windows-gnu

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install MinGW
        run: sudo apt-get install -y mingw-w64

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload
        uses: actions/upload-artifact@v3
        with:
          name: windows-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/tron-vanity.exe
```

---

## 完整工作流

### 从零开始编译 Windows 版本

```bash
# 1. 克隆项目
git clone https://github.com/yourusername/CoinAddress.git
cd CoinAddress

# 2. 安装 Rust（如果还没有）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. 安装 MinGW（macOS 示例）
brew install mingw-w64

# 4. 添加编译目标
rustup target add x86_64-pc-windows-gnu aarch64-pc-windows-gnu

# 5. 执行编译（选项 A：自动）
chmod +x build-windows.sh
./build-windows.sh

# 或编译特定版本（选项 B：手动 x86_64）
export CC=x86_64-w64-mingw32-gcc
export CXX=x86_64-w64-mingw32-g++
cargo build --release --target x86_64-pc-windows-gnu

# 6. 验证输出
ls -lh dist/tron-vanity_*.zip
```

---

## 常见问题

**Q: 为什么要使用 MinGW-w64 而不是 MSVC？**
A: MinGW-w64 可以从 Linux/macOS 交叉编译，而 MSVC 只能在 Windows 上使用。

**Q: 编译的 .exe 需要特殊的运行库吗？**
A: 如果使用 `-static-libgcc`，则不需要任何额外的运行库。

**Q: 能否编译 ARM64 版本在 x86_64 Windows 上运行？**
A: 不能。ARM64 二进制只能在 ARM64 Windows 上运行。

**Q: 如何调试编译问题？**
A: 使用 `RUSTFLAGS="-Z print-link-args" cargo build ...` 查看完整的编译命令。

---

## 推荐工具链

### macOS

```bash
brew install mingw-w64 llvm
```

### Linux (Ubuntu)

```bash
sudo apt-get install mingw-w64 mingw-w64-tools mingw-w64-common
```

### 验证安装

```bash
./setup-mingw.sh
```

---

## 相关资源

- [MinGW-w64 官方网站](https://www.mingw-w64.org/)
- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [Cargo 目标说明](https://doc.rust-lang.org/cargo/guide/build-cache.html)

---

**最后更新**: 2024-12-07
**版本**: 0.2.0

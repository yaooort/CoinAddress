# macOS ARM64 MinGW 工具链安装指南

## 问题

在 macOS 上，默认的 Homebrew `mingw-w64` 包只提供 **x86_64** 和 **i686** 架构支持，不包括 **aarch64 (ARM64)** 支持。

```
✗ aarch64-w64-mingw32-gcc 未安装
✗ aarch64-w64-mingw32-g++ 未安装
✗ aarch64-w64-mingw32-ar 未安装
```

## 解决方案

### ✅ 方案 1: 使用 Fork 的工具链 (推荐 - 最简单)

```bash
# 添加第三方 tap
brew tap esolitos/ipa https://github.com/esolitos/homebrew-ipa

# 安装包含 ARM64 支持的 mingw-w64
brew install mingw-w64-arm64

# 验证
aarch64-w64-mingw32-gcc --version
```

**优点**: 简单快速，预编译的二进制文件  
**缺点**: 依赖第三方维护

---

### ⚠️ 方案 2: 从源码编译 (较复杂 - 耗时)

如果第三方工具链不可用，可以从源码编译：

```bash
# 克隆 mingw-w64 源码
cd /tmp
git clone https://github.com/mingw-w64/mingw-w64

cd mingw-w64/mingw-w64-headers

# 配置为 ARM64 目标
./configure --host=aarch64-w64-mingw32 --prefix=/usr/local/mingw32-arm64

# 编译并安装
make
sudo make install

# 验证
aarch64-w64-mingw32-gcc --version
```

**优点**: 完全控制，最新版本  
**缺点**: 编译耗时 (30-60 分钟)，需要开发工具

---

### 💡 方案 3: 使用 LLVM/Clang (替代方案)

如果只是偶尔需要编译 ARM64，可以使用 LLVM 替代：

```bash
# 安装 LLVM
brew install llvm

# 配置 .cargo/config.toml
# [target.aarch64-pc-windows-gnu]
# linker = "aarch64-w64-mingw32-gcc"  # 可替换为 clang
```

---

### 🎯 方案 4: 跳过 ARM64，仅编译 x86_64 (最常见)

**大多数 Windows 用户使用 x86_64 架构**。如果不需要 ARM64 支持，可以跳过：

```bash
# 仅添加 x86_64 目标
rustup target add x86_64-pc-windows-gnu

# 编译 x86_64 版本
cargo build --release --target x86_64-pc-windows-gnu
```

---

## 当前状态

✅ **x86_64 编译工具**: 已安装并就绪  
⚠️ **ARM64 编译工具**: 未安装 (可选)

## 立即开始

### 1️⃣ 快速编译 x86_64 版本

```bash
# 添加 x86_64 编译目标
rustup target add x86_64-pc-windows-gnu

# 编译
./build-windows.sh

# 或手动编译
export CC=x86_64-w64-mingw32-gcc
export CXX=x86_64-w64-mingw32-g++
cargo build --release --target x86_64-pc-windows-gnu
```

### 2️⃣ 如果需要 ARM64 支持

```bash
# 方案 1 (推荐): 安装第三方工具链
brew tap esolitos/ipa https://github.com/esolitos/homebrew-ipa
brew install mingw-w64-arm64

# 验证
./setup-mingw.sh

# 添加 ARM64 编译目标
rustup target add aarch64-pc-windows-gnu

# 编译 ARM64 版本
cargo build --release --target aarch64-pc-windows-gnu
```

---

## 故障排查

### 问题: "formula not found" or tap 不工作

```bash
# 清除缓存
brew update

# 重新尝试
brew tap esolitos/ipa https://github.com/esolitos/homebrew-ipa
brew install mingw-w64-arm64
```

### 问题: 编译 ARM64 时出错

检查工具链是否正确安装：

```bash
which aarch64-w64-mingw32-gcc
aarch64-w64-mingw32-gcc --version
```

### 问题: 性能缓慢

- 使用 LTO 优化: 在 `.cargo/config.toml` 中已配置
- 增量编译: `cargo build -j 4` 使用多线程
- Release 优化: 已在 `Cargo.toml` 中配置

---

## 相关命令

```bash
# 验证安装
./setup-mingw.sh

# 编译 Windows 版本
./build-windows.sh

# 编译所有平台
./quick-build.sh

# 手动编译 x86_64
export CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++
cargo build --release --target x86_64-pc-windows-gnu

# 手动编译 ARM64 (需要先安装工具链)
export CC=aarch64-w64-mingw32-gcc CXX=aarch64-w64-mingw32-g++
cargo build --release --target aarch64-pc-windows-gnu
```

---

## 总结

| 方案                | 难度        | 时间       | 推荐度     |
| ------------------- | ----------- | ---------- | ---------- |
| 方案 1: Fork 工具链 | ⭐ 简单     | 5 分钟     | ⭐⭐⭐⭐⭐ |
| 方案 2: 源码编译    | ⭐⭐⭐ 复杂 | 30-60 分钟 | ⭐⭐       |
| 方案 3: LLVM 替代   | ⭐⭐ 中等   | 10 分钟    | ⭐⭐⭐     |
| 方案 4: 跳过 ARM64  | ⭐ 简单     | 立即       | ⭐⭐⭐⭐   |

**建议**: 先使用方案 1 或方案 4，足以满足大多数用户需求。

---

最后更新: 2024-12-07  
状态: ✅ x86_64 就绪，⚠️ ARM64 可选

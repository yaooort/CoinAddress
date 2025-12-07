# 安装和配置指南 | Installation & Setup Guide

## 📋 系统要求

### 最低配置

- **操作系统**: macOS 10.15+, Ubuntu 20.04+, Windows 10+
- **内存**: 4GB RAM
- **磁盘**: 500MB（源代码+编译后）
- **CPU**: 双核处理器

### 推荐配置

- **操作系统**: macOS 12+, Ubuntu 22.04 LTS, Windows 11
- **内存**: 16GB+ RAM
- **磁盘**: 1GB+ 可用空间
- **CPU**: 8 核+ 处理器（AMD Ryzen 5000 系列或 Intel 10 代+）

## 🚀 安装步骤

### 步骤 1: 安装 Rust

如果尚未安装 Rust，请访问 https://rustup.rs/ 并按照说明安装。

**macOS/Linux**:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows**:
下载并运行 https://win.rustup.rs/x86_64

验证安装：

```bash
rustc --version
cargo --version
```

### 步骤 2: 克隆或下载项目

```bash
# 如果有 Git
git clone <项目地址>
cd CoinAddress

# 或直接下载 ZIP 文件并解压
```

### 步骤 3: 编译项目

**使用快速启动脚本（推荐）**:

```bash
chmod +x quick-start.sh
./quick-start.sh
```

**手动编译**:

```bash
cargo build --release
```

编译会需要 1-5 分钟（取决于网络速度和硬件）。

### 步骤 4: 运行程序

**方式 1: 使用快速启动脚本**

```bash
./quick-start.sh
```

**方式 2: 直接运行二进制文件**

```bash
./target/release/tron-vanity
```

**方式 3: 使用 cargo 运行**

```bash
cargo run --release
```

## ⚙️ 配置指南

### 环境变量（可选）

```bash
# 设置输出文件位置
export TRON_OUTPUT_FILE="/path/to/results.txt"

# 设置线程数
export TRON_THREADS=16

# 设置批处理大小
export TRON_BATCH_SIZE=2000
```

### 高级配置

运行程序时选择选项 4（高级设置）进行实时配置：

```
╔═══════════════════════════════════════════════════════╗
║              高级设置 (Advanced Settings)             ║
╚═══════════════════════════════════════════════════════╝

保存所有生成的地址? (Save all addresses? y/n): y
线程数 (Number of threads, default 8): 16
批处理大小 (Batch size, default 1000): 2000
```

### 推荐配置参数

| 场景     | 线程数 | 批大小 | 保存所有 |
| -------- | ------ | ------ | -------- |
| 快速查找 | N 核   | 1000   | 否       |
| 大量收集 | N 核   | 5000   | 是       |
| 低功耗   | N/2    | 500    | 否       |
| 极限性能 | N      | 10000  | 否       |

_N = CPU 核心数_

## 📱 多平台部署

### macOS

```bash
# 使用 Homebrew 安装 Rust（可选）
brew install rustup-init

# 编译并创建快捷方式
cargo build --release
ln -s "$(pwd)/target/release/tron-vanity" /usr/local/bin/tron-vanity

# 使用
tron-vanity
```

### Linux (Ubuntu/Debian)

```bash
# 安装依赖
sudo apt-get install build-essential curl

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 编译
cargo build --release

# 创建快捷方式
sudo cp target/release/tron-vanity /usr/local/bin/

# 使用
tron-vanity
```

### Windows

```powershell
# 下载并安装 Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/

# 安装 Rust
# https://win.rustup.rs/x86_64

# 编译
cargo build --release

# 运行
.\target\release\tron-vanity.exe
```

### Docker（可选）

创建 `Dockerfile`:

```dockerfile
FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/tron-vanity"]
```

构建和运行：

```bash
docker build -t tron-vanity .
docker run -it tron-vanity
```

## 🔧 故障排除

### 问题 1: 编译失败

**错误**: `error: linker 'cc' not found`

**解决方案**:

```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install build-essential

# Windows
安装 Visual Studio Build Tools
```

### 问题 2: 编译很慢

**原因**: 首次编译需要下载所有依赖

**解决方案**:

- 等待编译完成（通常 5-10 分钟）
- 后续运行 `cargo build --release` 会快得多
- 使用 `cargo build -j 1` 限制并行编译

### 问题 3: 运行时崩溃

**错误**: `thread 'main' panicked`

**解决方案**:

```bash
# 运行调试版本查看更多信息
RUST_BACKTRACE=1 cargo run

# 或
export RUST_LOG=debug
./target/release/tron-vanity
```

### 问题 4: 内存不足

**症状**: 程序被 OOM 杀死或变得很慢

**解决方案**:

```
选择选项 4（高级设置）
批处理大小: 500（从 1000 降低）
线程数: 4（从 8 降低）
```

### 问题 5: macOS 安全警告

**警告**: "tron-vanity" cannot be opened because it is from an unidentified developer

**解决方案**:

```bash
# 方法 1: 允许执行
xattr -d com.apple.quarantine ./target/release/tron-vanity

# 方法 2: 使用 Gatekeeper 覆盖
spctl --add --label "TRON" ./target/release/tron-vanity
```

## 📊 性能优化

### 1. 启用 CPU 性能模式

**Linux**:

```bash
# 切换到性能模式
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

**macOS**:

```bash
# 禁用节能模式
sudo nvram boot-args="serverperfmode=1"
sudo reboot
```

### 2. 调整系统限制

**Linux**:

```bash
# 增加最大文件描述符
ulimit -n 65536

# 增加最大进程数
ulimit -u 4096
```

**macOS**:

```bash
# 编辑 /etc/launchd.conf
echo "limit maxfiles 65536 65536" | sudo tee -a /etc/launchd.conf
sudo launchctl limit maxfiles 65536 65536
```

### 3. 优化线程配置

查看 CPU 核心数：

```bash
# Linux/macOS
nproc

# Windows (PowerShell)
[Environment]::ProcessorCount
```

建议线程数 = CPU 核心数（最大性能）

## 🧹 清理和卸载

### 删除编译文件

```bash
cargo clean
```

### 删除项目

```bash
rm -rf CoinAddress/
```

### 删除全局安装

```bash
rm /usr/local/bin/tron-vanity
```

## 🆘 获取帮助

### 查看帮助信息

```bash
cargo --help
cargo build --help
cargo run --help
```

### 检查依赖

```bash
# 查看所有依赖
cargo tree

# 检查更新
cargo update --dry-run

# 更新依赖
cargo update
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_tron_address

# 详细输出
cargo test -- --nocapture
```

### 查看日志

```bash
# 详细日志
RUST_LOG=debug ./target/release/tron-vanity

# 跟踪日志
RUST_BACKTRACE=full ./target/release/tron-vanity
```

## 📚 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Cargo 指南](https://doc.rust-lang.org/cargo/)
- [TRON 开发文档](https://developers.tron.network/)
- [Rust 最佳实践](https://rust-lang.org/what/wg-libs/)

## 🔐 安全提示

1. **总是在离线环境运行**
2. **自己编译源代码**
3. **不要共享私钥或助记词**
4. **定期备份结果文件**
5. **使用安全的计算机**

---

**最后更新**: 2025 年 12 月 7 日  
**版本**: 0.1.0

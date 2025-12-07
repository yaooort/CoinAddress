#!/bin/bash

# MinGW-w64 安装和验证脚本
# 支持 macOS、Linux、Windows
# 处理 x86_64 和 ARM64 编译器

set -e

echo "═══════════════════════════════════════════════════════════════════════════"
echo "MinGW-w64 工具链检查和安装"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""

# 检测操作系统
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "📋 系统信息"
echo "  操作系统: $OS"
echo "  架构: $ARCH"
echo ""

# 定义需要检查的工具
TOOLS_X64=("x86_64-w64-mingw32-gcc" "x86_64-w64-mingw32-g++" "x86_64-w64-mingw32-ar")
TOOLS_ARM64=("aarch64-w64-mingw32-gcc" "aarch64-w64-mingw32-g++" "aarch64-w64-mingw32-ar")

# 检查工具函数
check_tool() {
    local tool=$1
    if command -v "$tool" &> /dev/null; then
        local version=$("$tool" --version | head -n 1)
        echo "✅ $tool"
        echo "   $version"
        return 0
    else
        echo "✗ $tool 未安装"
        return 1
    fi
}

echo "🔍 检查 MinGW-w64 工具链"
echo ""
echo "Windows x86_64 编译工具:"
X64_OK=true
for tool in "${TOOLS_X64[@]}"; do
    if ! check_tool "$tool"; then
        X64_OK=false
    fi
done
echo ""
echo "Windows ARM64 编译工具:"
ARM64_OK=true
for tool in "${TOOLS_ARM64[@]}"; do
    if ! check_tool "$tool"; then
        ARM64_OK=false
    fi
done
echo ""

# 安装提示
echo "📦 安装状态和指南"
echo ""

if [ "$X64_OK" = true ]; then
    echo "✅ x86_64 编译工具完整可用"
else
    echo "⚠️  x86_64 编译工具缺失"
fi

if [ "$ARM64_OK" = true ]; then
    echo "✅ ARM64 编译工具完整可用"
else
    echo "⚠️  ARM64 编译工具缺失 (可选，使用 LLVM/Clang 替代)"
fi
echo ""

case $OS in
    Darwin)
        echo "macOS 系统 - 使用 Homebrew 安装:"
        echo ""
        echo "1️⃣  安装基础 MinGW-w64 (x86_64):"
        echo "  $ brew install mingw-w64"
        echo ""
        if [ "$ARM64_OK" = false ]; then
            echo "2️⃣  ARM64 支持选项:"
            echo ""
            echo "  选项 A - 使用编译的工具链 (推荐):"
            echo "  $ brew tap esolitos/ipa https://github.com/esolitos/homebrew-ipa"
            echo "  $ brew install mingw-w64-arm64"
            echo ""
            echo "  选项 B - 从源码编译 (耗时):"
            echo "  $ cd /tmp && git clone https://github.com/mingw-w64/mingw-w64"
            echo "  $ cd mingw-w64/mingw-w64-headers && ./configure \\"
            echo "    --host=aarch64-w64-mingw32 --prefix=/usr/local/mingw32-arm64"
            echo "  $ make && make install"
            echo ""
            echo "  选项 C - 使用 LLVM 替代 (更简单):"
            echo "  $ brew install llvm"
            echo "  (可在 .cargo/config.toml 中配置)"
        fi
        ;;
    Linux)
        echo "Linux 系统 - 使用包管理器安装:"
        echo ""
        # 检测 Linux 发行版
        if [ -f /etc/debian_version ]; then
            echo "Debian/Ubuntu:"
            echo "  $ sudo apt-get update"
            echo "  $ sudo apt-get install mingw-w64 mingw-w64-tools"
            if [ "$ARM64_OK" = false ]; then
                echo ""
                echo "  为了获得完整 ARM64 支持:"
                echo "  $ sudo apt-get install mingw-w64-arm64-dev"
            fi
        elif [ -f /etc/redhat-release ]; then
            echo "Fedora/RHEL:"
            echo "  $ sudo dnf install mingw-w64-gcc mingw-w64-gcc-c++"
            if [ "$ARM64_OK" = false ]; then
                echo ""
                echo "  为了获得完整 ARM64 支持:"
                echo "  $ sudo dnf install mingw-w64-generic-devel.aarch64"
            fi
        elif [ -f /etc/arch-release ]; then
            echo "Arch Linux:"
            echo "  $ sudo pacman -S mingw-w64"
        else
            echo "通用 Linux:"
            echo "  $ sudo apt-get install mingw-w64  # Debian/Ubuntu"
            echo "  $ sudo dnf install mingw-w64-gcc  # Fedora"
            echo "  $ sudo pacman -S mingw-w64        # Arch"
        fi
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "Windows 系统 - 使用 MSVC 或 MinGW:"
        echo ""
        echo "推荐: Visual Studio Community (includes MSVC):"
        echo "  https://visualstudio.microsoft.com/"
        echo ""
        echo "或手动安装 MinGW:"
        echo "  https://www.mingw-w64.org/downloads/"
        ;;
    *)
        echo "未识别的操作系统: $OS"
        echo ""
        echo "请访问 https://www.mingw-w64.org/ 获取安装指南"
        ;;
esac
echo ""

echo "═══════════════════════════════════════════════════════════════════════════"
echo "Rust 编译目标设置"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""
echo "需要添加 Rust 编译目标:"
echo ""
if [ "$X64_OK" = true ]; then
    echo "✅ 添加 x86_64 目标:"
    echo "  $ rustup target add x86_64-pc-windows-gnu"
else
    echo "⚠️  跳过 x86_64 (工具链缺失)"
fi
echo ""
if [ "$ARM64_OK" = true ]; then
    echo "✅ 添加 ARM64 目标:"
    echo "  $ rustup target add aarch64-pc-windows-gnu"
else
    echo "⚠️  ARM64 工具缺失 - 备选方案:"
    echo ""
    echo "  方案 1: 在 Windows 上编译 ARM64 版本"
    echo "  (在 Windows 系统上运行 'cargo build --target aarch64-pc-windows-gnu')"
    echo ""
    echo "  方案 2: 使用 LLVM 编译到 Windows ARM64"
    echo "  (需要在 .cargo/config.toml 中配置)"
    echo ""
    echo "  方案 3: 仅编译 x86_64 Windows 版本 (最常见)"
    echo "  (大多数 Windows 用户使用 x86_64)"
fi
echo ""
echo "验证:"
echo "  $ rustup target list | grep windows-gnu"
echo ""

echo "═══════════════════════════════════════════════════════════════════════════"
if [ "$X64_OK" = true ]; then
    echo "✅ x86_64 编译工具就绪"
else
    echo "❌ 需要安装工具 (见上方指南)"
fi
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""
echo "下一步:"
echo "  1. 按上述指南安装必要的工具"
echo "  2. 运行此脚本验证安装: ./setup-mingw.sh"
echo "  3. 添加 Rust 编译目标: rustup target add x86_64-pc-windows-gnu"
echo "  4. 编译 Windows 版本: ./build-windows.sh"
echo ""
echo "✨ 配置完成！"

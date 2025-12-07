#!/bin/bash

# TRON Vanity 设置验证脚本

set -e

RESET='\033[0m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'

check_mark="${GREEN}✓${RESET}"
cross_mark="${RED}✗${RESET}"

echo -e "${BLUE}======================================${RESET}"
echo -e "${BLUE}TRON Vanity 环境验证${RESET}"
echo -e "${BLUE}======================================${RESET}"
echo ""

# 检查Rust
echo -n "检查 Rust... "
if command -v rustc &> /dev/null; then
    rustc_version=$(rustc --version | awk '{print $2}')
    echo -e "${check_mark} $rustc_version"
else
    echo -e "${cross_mark} 未安装"
    exit 1
fi

# 检查Cargo
echo -n "检查 Cargo... "
if command -v cargo &> /dev/null; then
    cargo_version=$(cargo --version | awk '{print $2}')
    echo -e "${check_mark} $cargo_version"
else
    echo -e "${cross_mark} 未安装"
    exit 1
fi

# 检查Python
echo -n "检查 Python3... "
if command -v python3 &> /dev/null; then
    python_version=$(python3 --version | awk '{print $2}')
    echo -e "${check_mark} $python_version"
else
    echo -e "${cross_mark} 未安装"
fi

# 检查Git
echo -n "检查 Git... "
if command -v git &> /dev/null; then
    git_version=$(git --version | awk '{print $3}')
    echo -e "${check_mark} $git_version"
else
    echo -e "${cross_mark} 未安装"
fi

echo ""
echo -e "${BLUE}编译目标:${RESET}"

targets=(
    "x86_64-unknown-linux-gnu:Linux x86_64"
    "aarch64-unknown-linux-gnu:Linux ARM64"
    "x86_64-pc-windows-gnu:Windows x86_64"
    "aarch64-pc-windows-gnu:Windows ARM64"
    "x86_64-apple-darwin:macOS x86_64"
    "aarch64-apple-darwin:macOS ARM64"
)

for target_info in "${targets[@]}"; do
    IFS=':' read -r target name <<< "$target_info"
    echo -n "  $name... "
    if rustup target list | grep "$target" | grep -q installed; then
        echo -e "${check_mark}"
    else
        echo -e "${YELLOW}未安装${RESET}"
    fi
done

echo ""
echo -e "${BLUE}项目文件:${RESET}"

files=(
    "src/lib.rs:核心库"
    "src/gui.rs:GUI应用"
    "src/main.rs:主程序入口"
    "Cargo.toml:依赖配置"
    "build-cross.sh:交叉编译脚本"
    "quick-build.sh:快速编译脚本"
    "generate-logo.py:Logo生成器"
    "assets/logos/logo.svg:主Logo"
)

for file_info in "${files[@]}"; do
    IFS=':' read -r file desc <<< "$file_info"
    echo -n "  $desc... "
    if [ -f "$file" ]; then
        size=$(du -h "$file" | awk '{print $1}')
        echo -e "${check_mark} ($size)"
    else
        echo -e "${cross_mark} 未找到"
    fi
done

echo ""
echo -e "${BLUE}构建脚本权限:${RESET}"

scripts=("build-cross.sh" "quick-build.sh" "generate-logo.py" "deploy-resources.sh")

for script in "${scripts[@]}"; do
    echo -n "  $script... "
    if [ -x "$script" ]; then
        echo -e "${check_mark}"
    else
        echo -e "${YELLOW}需要添加执行权限${RESET}"
        chmod +x "$script" 2>/dev/null
    fi
done

echo ""
echo -e "${BLUE}磁盘空间:${RESET}"

disk_free=$(df -h . | tail -1 | awk '{print $4}')
echo "  可用空间: $disk_free"

echo ""
echo -e "${BLUE}======================================${RESET}"
echo -e "${GREEN}✓ 环境验证完成！${RESET}"
echo -e "${BLUE}======================================${RESET}"
echo ""
echo "下一步:"
echo "  • 快速编译: ./quick-build.sh"
echo "  • 生成Logo: python3 generate-logo.py"
echo "  • 查看帮助: ./build-cross.sh --help"
echo ""

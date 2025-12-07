#!/bin/bash

# Windows 交叉编译脚本 - 使用 MinGW-w64 工具链

set -e

VERSION="0.2.0"
PROJECT_NAME="tron-vanity"
BUILD_DIR="dist"

echo "🪟 Windows TRON Vanity 交叉编译脚本"
echo "======================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 检查 MinGW 工具
check_mingw() {
    echo "检查 MinGW 工具..."
    
    local x64_ok=true
    local arm64_ok=true
    
    # 检查 x86_64 工具
    if ! command -v "x86_64-w64-mingw32-gcc" &> /dev/null; then
        echo -e "${RED}✗${NC} Windows x86_64 GCC 未安装"
        x64_ok=false
    else
        echo -e "${GREEN}✓${NC} Windows x86_64 GCC 已安装"
    fi
    
    if ! command -v "x86_64-w64-mingw32-g++" &> /dev/null; then
        echo -e "${RED}✗${NC} Windows x86_64 G++ 未安装"
        x64_ok=false
    else
        echo -e "${GREEN}✓${NC} Windows x86_64 G++ 已安装"
    fi
    
    # 检查 ARM64 工具 (可选)
    if ! command -v "aarch64-w64-mingw32-gcc" &> /dev/null; then
        echo -e "${YELLOW}⚠${NC}  Windows ARM64 GCC 未安装 (可选)"
        arm64_ok=false
    else
        echo -e "${GREEN}✓${NC} Windows ARM64 GCC 已安装"
    fi
    
    if ! command -v "aarch64-w64-mingw32-g++" &> /dev/null; then
        echo -e "${YELLOW}⚠${NC}  Windows ARM64 G++ 未安装 (可选)"
        arm64_ok=false
    else
        echo -e "${GREEN}✓${NC} Windows ARM64 G++ 已安装"
    fi
    
    echo ""
    
    # x86_64 是必需的
    if [ "$x64_ok" = false ]; then
        echo -e "${RED}❌ x86_64 编译工具缺失，无法继续${NC}"
        echo "  运行: ./setup-mingw.sh"
        return 1
    fi
    
    # 保存 ARM64 状态
    export ARM64_AVAILABLE=$arm64_ok
    
    if [ "$arm64_ok" = false ]; then
        echo -e "${YELLOW}⚠️  ARM64 工具不可用，仅编译 x86_64${NC}"
        echo "   如需 ARM64 支持，请参考 MACOS-ARM64-MINGW-GUIDE.md"
    fi
    
    return 0
}

# 编译 Windows x86_64
compile_windows_x64() {
    echo ""
    echo -e "${BLUE}编译 Windows x86_64...${NC}"
    
    export CC="x86_64-w64-mingw32-gcc"
    export CXX="x86_64-w64-mingw32-g++"
    export AR="x86_64-w64-mingw32-ar"
    
    cargo build --release --target x86_64-pc-windows-gnu 2>&1 | grep -E "error|Finished|Compiling" || true
    
    if [ -f "target/x86_64-pc-windows-gnu/release/${PROJECT_NAME}.exe" ]; then
        echo -e "${GREEN}✓ Windows x86_64 编译成功${NC}"
        return 0
    else
        echo -e "${RED}✗ Windows x86_64 编译失败${NC}"
        return 1
    fi
}

# 编译 Windows ARM64
compile_windows_arm64() {
    # 检查 ARM64 工具是否可用
    if [ "$ARM64_AVAILABLE" != "true" ]; then
        echo ""
        echo -e "${YELLOW}⚠️  ARM64 编译工具不可用，跳过 ARM64 编译${NC}"
        echo "   参考: MACOS-ARM64-MINGW-GUIDE.md"
        return 0
    fi
    
    echo ""
    echo -e "${BLUE}编译 Windows ARM64...${NC}"
    
    export CC="aarch64-w64-mingw32-gcc"
    export CXX="aarch64-w64-mingw32-g++"
    export AR="aarch64-w64-mingw32-ar"
    
    cargo build --release --target aarch64-pc-windows-gnu 2>&1 | grep -E "error|Finished|Compiling" || true
    
    if [ -f "target/aarch64-pc-windows-gnu/release/${PROJECT_NAME}.exe" ]; then
        echo -e "${GREEN}✓ Windows ARM64 编译成功${NC}"
        return 0
    else
        echo -e "${RED}✗ Windows ARM64 编译失败${NC}"
        return 1
    fi
}

# 打包函数
package_windows() {
    local target=$1
    local arch=$2
    
    local bin_path="target/$target/release/${PROJECT_NAME}.exe"
    
    if [ ! -f "$bin_path" ]; then
        echo -e "${RED}✗ 二进制文件不存在: $bin_path${NC}"
        return 1
    fi
    
    mkdir -p "$BUILD_DIR"
    
    local package_name="${PROJECT_NAME}_${VERSION}_windows-${arch}"
    local package_dir="$BUILD_DIR/$package_name"
    
    mkdir -p "$package_dir"
    
    # 复制二进制
    cp "$bin_path" "$package_dir/"
    
    # 复制文档
    cp README.md "$package_dir/" 2>/dev/null || true
    cp PACKAGING.md "$package_dir/" 2>/dev/null || true
    
    # 创建运行脚本
    cat > "$package_dir/run.bat" << 'EOF'
@echo off
cd /d "%~dp0"
tron-vanity.exe
pause
EOF
    
    # 创建无暂停版本
    cat > "$package_dir/run-silent.bat" << 'EOF'
@echo off
cd /d "%~dp0"
start tron-vanity.exe
EOF
    
    # 打包为 ZIP
    cd "$BUILD_DIR"
    zip -q -r "$package_name.zip" "$package_name"
    rm -rf "$package_name"
    cd ..
    
    local size=$(du -h "$BUILD_DIR/${package_name}.zip" | awk '{print $1}')
    echo -e "${GREEN}✓ 已打包: $BUILD_DIR/${package_name}.zip ($size)${NC}"
}

# 主函数
main() {
    mkdir -p "$BUILD_DIR"
    
    # 检查 MinGW
    if ! check_mingw; then
        echo ""
        echo -e "${RED}❌ x86_64 编译工具未完整安装${NC}"
        echo "请先运行: ./setup-mingw.sh"
        exit 1
    fi
    
    echo ""
    echo -e "${BLUE}开始编译...${NC}"
    
    X64_SUCCESS=false
    ARM64_SUCCESS=false
    
    # 编译 x86_64
    if compile_windows_x64; then
        if package_windows "x86_64-pc-windows-gnu" "x86_64"; then
            X64_SUCCESS=true
        fi
    fi
    
    # 编译 ARM64 (如果工具可用)
    if compile_windows_arm64; then
        if [ "$ARM64_AVAILABLE" = "true" ]; then
            if package_windows "aarch64-pc-windows-gnu" "arm64"; then
                ARM64_SUCCESS=true
            fi
        fi
    fi
    
    echo ""
    echo -e "${BLUE}════════════════════════════════════${NC}"
    if [ "$X64_SUCCESS" = true ]; then
        echo -e "${GREEN}✅ x86_64 编译完成！${NC}"
        if [ "$ARM64_SUCCESS" = true ]; then
            echo -e "${GREEN}✅ ARM64 编译完成！${NC}"
        fi
    else
        echo -e "${RED}❌ 编译失败${NC}"
        exit 1
    fi
    echo -e "${BLUE}════════════════════════════════════${NC}"
    echo ""
    echo "📦 输出文件:"
    ls -lh "$BUILD_DIR" | grep windows | awk '{printf "   %-50s %8s\n", $9, $5}'
    echo ""
}

main "$@"

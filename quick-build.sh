#!/bin/bash
# 快速交叉编译脚本 - 一键生成所有平台的二进制文件

set -e

echo "🚀 TRON Vanity 快速编译器"
echo "======================================"
echo ""

VERSION="0.2.0"
BUILD_DIR="dist"

# 检查必要的工具
check_tools() {
    echo "✓ 检查工具..."
    
    if ! command -v rustc &> /dev/null; then
        echo "❌ 需要安装 Rust: https://rustup.rs/"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo "❌ 需要安装 Cargo"
        exit 1
    fi
    
    # 检查gzip（macOS x86_64需要）
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if ! command -v gzip &> /dev/null; then
            echo "❌ 需要 gzip 工具"
            exit 1
        fi
    fi
}

# 显示平台信息
show_system_info() {
    echo "📊 系统信息:"
    echo "  操作系统: $(uname -s)"
    echo "  架构: $(uname -m)"
    echo "  Rust 版本: $(rustc --version)"
    echo ""
}

# 添加所有编译目标
add_all_targets() {
    echo "📦 添加编译目标..."
    
    local targets=(
        "x86_64-unknown-linux-gnu"
        "aarch64-unknown-linux-gnu"
        "x86_64-pc-windows-gnu"
        "aarch64-pc-windows-gnu"
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )
    
    for target in "${targets[@]}"; do
        rustup target add "$target" 2>&1 | grep -i "added\|already" || true
    done
    
    echo "✓ 所有目标已添加"
    echo ""
}

# 编译函数
compile_target() {
    local target=$1
    local display=$2
    
    echo "⚙️  编译 $display ($target)..."
    
    if cargo build --release --target "$target" 2>&1 | grep -E "error|Error" > /dev/null; then
        echo "❌ $display 编译失败"
        return 1
    else
        echo "✅ $display 编译成功"
        return 0
    fi
}

# 打包函数
package_target() {
    local target=$1
    local display=$2
    local os=$3
    
    local bin_name="tron-vanity"
    if [[ "$target" == *"windows"* ]]; then
        bin_name="tron-vanity.exe"
    fi
    
    local bin_path="target/$target/release/$bin_name"
    
    if [ ! -f "$bin_path" ]; then
        echo "⚠️  未找到二进制: $bin_path"
        return 1
    fi
    
    mkdir -p "$BUILD_DIR"
    local package_name="${os}-${VERSION}"
    local size=$(du -h "$bin_path" | awk '{print $1}')
    
    if [[ "$os" == *"windows"* ]]; then
        cp "$bin_path" "$BUILD_DIR/${package_name}.exe"
        echo "  📦 已保存: $BUILD_DIR/${package_name}.exe (大小: $size)"
    else
        cp "$bin_path" "$BUILD_DIR/$package_name"
        chmod +x "$BUILD_DIR/$package_name"
        echo "  📦 已保存: $BUILD_DIR/$package_name (大小: $size)"
    fi
}

# 主编译流程
main() {
    check_tools
    show_system_info
    
    mkdir -p "$BUILD_DIR"
    rm -f "$BUILD_DIR"/* 2>/dev/null || true
    
    add_all_targets
    
    echo "🔨 开始编译..."
    echo ""
    
    # 支持的所有平台
    local targets=(
        "x86_64-unknown-linux-gnu:Linux x86_64:linux"
        "aarch64-unknown-linux-gnu:Linux ARM64:linux-arm"
        "x86_64-pc-windows-gnu:Windows x86_64:windows"
        "aarch64-pc-windows-gnu:Windows ARM64:windows-arm"
    )
    
    # macOS仅在macOS上编译
    if [[ "$OSTYPE" == "darwin"* ]]; then
        targets+=(
            "x86_64-apple-darwin:macOS x86_64:macos"
            "aarch64-apple-darwin:macOS ARM64 (Apple Silicon):macos-arm"
        )
    else
        echo "⚠️  当前系统非macOS，跳过macOS编译"
        echo ""
    fi
    
    # 编译所有目标
    for target_info in "${targets[@]}"; do
        IFS=':' read -r target display os <<< "$target_info"
        if compile_target "$target" "$display"; then
            package_target "$target" "$display" "$os"
        fi
        echo ""
    done
    
    # 显示最终结果
    echo "======================================"
    echo "✨ 编译完成！"
    echo ""
    echo "📁 输出目录: $BUILD_DIR"
    echo ""
    echo "📋 生成的文件:"
    ls -lh "$BUILD_DIR" | tail -n +2 | awk '{printf "   %-40s %6s\n", $9, $5}'
    echo ""
    echo "💡 提示:"
    echo "   • 所有二进制文件都在 $BUILD_DIR 目录中"
    echo "   • 可以直接运行或分发使用"
    echo "   • 使用 ./build-cross.sh 脚本获得更多选项"
}

main "$@"

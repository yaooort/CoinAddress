#!/bin/bash

# TRON Vanity 地址生成器 - 交叉编译脚本
# 支持平台: Windows (x86_64, aarch64) | macOS (x86_64, aarch64) | Linux (x86_64, aarch64)

set -e

VERSION="0.2.0"
PROJECT_NAME="tron-vanity"
BUILD_DIR="dist"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

# 初始化构建目录
init_build_dir() {
    print_header "初始化构建环境"
    
    if [ -d "$BUILD_DIR" ]; then
        rm -rf "$BUILD_DIR"
    fi
    mkdir -p "$BUILD_DIR"
    print_success "构建目录已创建: $BUILD_DIR"
}

# 检查依赖
check_dependencies() {
    print_header "检查依赖"
    
    # 检查rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust 未安装，请先安装: https://rustup.rs/"
        exit 1
    fi
    print_success "Rust 已安装: $(rustc --version)"
    
    # 检查cargo
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo 未安装"
        exit 1
    fi
    print_success "Cargo 已安装: $(cargo --version)"
}

# 添加编译目标
add_targets() {
    print_header "添加编译目标"
    
    local targets=(
        "x86_64-unknown-linux-gnu"
        "aarch64-unknown-linux-gnu"
        "x86_64-pc-windows-gnu"
        "aarch64-pc-windows-gnu"
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )
    
    for target in "${targets[@]}"; do
        print_info "检查目标: $target"
        rustup target add "$target" 2>/dev/null || print_info "  已存在或跳过"
    done
    print_success "所有编译目标已添加"
}

# 交叉编译函数
cross_compile() {
    local target=$1
    local os=$2
    local arch=$3
    
    print_info "编译 $os/$arch ($target)..."
    
    cargo build --release --target "$target" 2>&1 | grep -E "error|Finished|Compiling" || true
    
    if [ -f "target/$target/release/$PROJECT_NAME" ] || [ -f "target/$target/release/$PROJECT_NAME.exe" ]; then
        print_success "$os/$arch 编译完成"
        return 0
    else
        print_error "$os/$arch 编译失败"
        return 1
    fi
}

# 打包函数
package_build() {
    local target=$1
    local os=$2
    local arch=$3
    
    local bin_name="$PROJECT_NAME"
    if [[ "$target" == *"windows"* ]]; then
        bin_name="$PROJECT_NAME.exe"
    fi
    
    local bin_path="target/$target/release/$bin_name"
    local package_name="${PROJECT_NAME}_${VERSION}_${os}-${arch}"
    local package_dir="$BUILD_DIR/$package_name"
    
    if [ ! -f "$bin_path" ]; then
        print_error "二进制文件不存在: $bin_path"
        return 1
    fi
    
    # 创建打包目录
    mkdir -p "$package_dir"
    
    # 复制二进制文件
    cp "$bin_path" "$package_dir/$bin_name"
    chmod +x "$package_dir/$bin_name" 2>/dev/null || true
    
    # 复制文档
    [ -f "README.md" ] && cp "README.md" "$package_dir/"
    [ -f "PACKAGING.md" ] && cp "PACKAGING.md" "$package_dir/"
    [ -f "LICENSE" ] && cp "LICENSE" "$package_dir/"
    
    # 创建启动脚本（非Windows）
    if [[ "$os" != "Windows" ]]; then
        cat > "$package_dir/run.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
./tron-vanity
EOF
        chmod +x "$package_dir/run.sh"
    else
        # Windows批处理脚本
        cat > "$package_dir/run.bat" << 'EOF'
@echo off
cd /d "%~dp0"
tron-vanity.exe
pause
EOF
    fi
    
    # 压缩打包
    cd "$BUILD_DIR"
    
    if [[ "$os" == "Windows" ]]; then
        # Windows 使用 zip
        zip -q -r "$package_name.zip" "$package_name"
        print_success "已打包: $package_name.zip"
    else
        # Unix-like 使用 tar.gz
        tar -czf "$package_name.tar.gz" "$package_name"
        print_success "已打包: $package_name.tar.gz"
    fi
    
    cd "$SCRIPT_DIR"
    rm -rf "$package_dir"
}

# 编译所有平台
build_all() {
    print_header "开始交叉编译所有平台"
    
    # Linux x86_64
    cross_compile "x86_64-unknown-linux-gnu" "Linux" "x86_64" && \
        package_build "x86_64-unknown-linux-gnu" "Linux" "x86_64"
    
    # Linux ARM64
    cross_compile "aarch64-unknown-linux-gnu" "Linux" "arm64" && \
        package_build "aarch64-unknown-linux-gnu" "Linux" "arm64"
    
    # Windows x86_64
    cross_compile "x86_64-pc-windows-gnu" "Windows" "x86_64" && \
        package_build "x86_64-pc-windows-gnu" "Windows" "x86_64"
    
    # Windows ARM64
    cross_compile "aarch64-pc-windows-gnu" "Windows" "arm64" && \
        package_build "aarch64-pc-windows-gnu" "Windows" "arm64"
    
    # macOS x86_64 (仅macOS可编译)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        cross_compile "x86_64-apple-darwin" "macOS" "x86_64" && \
            package_build "x86_64-apple-darwin" "macOS" "x86_64"
        
        # macOS ARM64 (Apple Silicon)
        cross_compile "aarch64-apple-darwin" "macOS" "arm64" && \
            package_build "aarch64-apple-darwin" "macOS" "arm64"
    else
        print_info "当前系统不是macOS，跳过macOS编译"
    fi
}

# 生成构建报告
generate_report() {
    print_header "构建完成"
    
    echo -e "${GREEN}生成的二进制文件:${NC}"
    ls -lh "$BUILD_DIR" 2>/dev/null || echo "没有生成文件"
    
    echo ""
    echo -e "${GREEN}构建统计:${NC}"
    local total=$(find "$BUILD_DIR" -type f | wc -l)
    echo "  总文件数: $total"
    
    local total_size=$(du -sh "$BUILD_DIR" 2>/dev/null | awk '{print $1}')
    echo "  总大小: $total_size"
}

# 清理
cleanup() {
    print_info "清理临时文件..."
    cargo clean
}

# 帮助信息
show_help() {
    cat << EOF
TRON Vanity 交叉编译脚本

用法: $0 [选项]

选项:
  --help          显示此帮助信息
  --clean         清理构建文件
  --deps          仅检查依赖
  --add-targets   仅添加编译目标
  --build-all     编译所有平台（默认）
  --linux-x64     仅编译 Linux x86_64
  --linux-arm     仅编译 Linux ARM64
  --win-x64       仅编译 Windows x86_64
  --win-arm       仅编译 Windows ARM64
  --macos-x64     仅编译 macOS x86_64
  --macos-arm     仅编译 macOS ARM64

示例:
  $0 --build-all          # 编译所有平台
  $0 --linux-x64          # 仅编译 Linux x86_64
  $0 --clean              # 清理所有构建文件

EOF
}

# 主逻辑
main() {
    if [ $# -eq 0 ]; then
        check_dependencies
        init_build_dir
        add_targets
        build_all
        generate_report
        return 0
    fi
    
    case "$1" in
        --help)
            show_help
            ;;
        --clean)
            cleanup
            ;;
        --deps)
            check_dependencies
            ;;
        --add-targets)
            add_targets
            ;;
        --build-all)
            check_dependencies
            init_build_dir
            add_targets
            build_all
            generate_report
            ;;
        --linux-x64)
            check_dependencies
            init_build_dir
            cross_compile "x86_64-unknown-linux-gnu" "Linux" "x86_64"
            package_build "x86_64-unknown-linux-gnu" "Linux" "x86_64"
            ;;
        --linux-arm)
            check_dependencies
            init_build_dir
            cross_compile "aarch64-unknown-linux-gnu" "Linux" "arm64"
            package_build "aarch64-unknown-linux-gnu" "Linux" "arm64"
            ;;
        --win-x64)
            check_dependencies
            init_build_dir
            cross_compile "x86_64-pc-windows-gnu" "Windows" "x86_64"
            package_build "x86_64-pc-windows-gnu" "Windows" "x86_64"
            ;;
        --win-arm)
            check_dependencies
            init_build_dir
            cross_compile "aarch64-pc-windows-gnu" "Windows" "arm64"
            package_build "aarch64-pc-windows-gnu" "Windows" "arm64"
            ;;
        --macos-x64)
            if [[ "$OSTYPE" != "darwin"* ]]; then
                print_error "只能在macOS上编译macOS版本"
                exit 1
            fi
            check_dependencies
            init_build_dir
            cross_compile "x86_64-apple-darwin" "macOS" "x86_64"
            package_build "x86_64-apple-darwin" "macOS" "x86_64"
            ;;
        --macos-arm)
            if [[ "$OSTYPE" != "darwin"* ]]; then
                print_error "只能在macOS上编译macOS版本"
                exit 1
            fi
            check_dependencies
            init_build_dir
            cross_compile "aarch64-apple-darwin" "macOS" "arm64"
            package_build "aarch64-apple-darwin" "macOS" "arm64"
            ;;
        *)
            print_error "未知选项: $1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"

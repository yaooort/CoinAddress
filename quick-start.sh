#!/usr/bin/env bash

# TRON 波场靓号生成器 - 快速启动指南

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$PROJECT_DIR/target/release/tron-vanity"
OUTPUT_FILE="$PROJECT_DIR/tron_vanity.txt"

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 显示欢迎信息
show_banner() {
    echo -e "${BLUE}"
    cat << "EOF"
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║      TRON 波场靓号生成器 | TRON Vanity Generator         ║
║                                                            ║
║         快速 • 高效 • 支持多线程并行处理                  ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

# 检查是否需要编译
check_and_compile() {
    if [ ! -f "$BINARY" ]; then
        echo -e "${YELLOW}❓ 找不到编译文件，正在编译...${NC}"
        echo "Building in Release mode..."
        cd "$PROJECT_DIR"
        cargo build --release
        echo -e "${GREEN}✓ 编译完成！${NC}"
    else
        echo -e "${GREEN}✓ 找到编译文件${NC}"
    fi
}

# 显示菜单
show_menu() {
    echo -e "${BLUE}请选择操作 | Please select an option:${NC}"
    echo ""
    echo -e "  ${GREEN}1${NC}  使用默认靓号   (Default Vanity Patterns)"
    echo -e "  ${GREEN}2${NC}  自定义靓号模式  (Custom Patterns)"
    echo -e "  ${GREEN}3${NC}  性能测试       (Benchmark Test)"
    echo -e "  ${GREEN}4${NC}  高级设置       (Advanced Settings)"
    echo -e "  ${GREEN}5${NC}  查看结果文件   (View Results)"
    echo -e "  ${GREEN}0${NC}  退出           (Exit)"
    echo ""
}

# 查看结果文件
view_results() {
    if [ -f "$OUTPUT_FILE" ]; then
        echo -e "${GREEN}最近的靓号结果 | Recent Results:${NC}"
        echo ""
        tail -100 "$OUTPUT_FILE"
    else
        echo -e "${YELLOW}❌ 还没有找到靓号 | No results yet${NC}"
    fi
}

# 显示统计信息
show_stats() {
    if [ -f "$OUTPUT_FILE" ]; then
        total=$(grep -c "^\[" "$OUTPUT_FILE" 2>/dev/null || echo "0")
        vanity=$(grep -c "^\[VANITY\]" "$OUTPUT_FILE" 2>/dev/null || echo "0")
        echo ""
        echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
        echo -e "总记录数 | Total Records: ${GREEN}$total${NC}"
        echo -e "靓号数量 | Vanity Addresses: ${YELLOW}$vanity${NC}"
        echo -e "结果文件 | Results File: ${GREEN}$OUTPUT_FILE${NC}"
        echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
    fi
}

# 主循环
main_loop() {
    while true; do
        echo ""
        show_menu
        read -p "选择 (Select): " choice
        
        case $choice in
            1)
                echo -e "${YELLOW}使用默认靓号模式启动... Starting with default patterns...${NC}"
                echo "1" | "$BINARY"
                ;;
            2)
                echo -e "${YELLOW}自定义靓号模式启动... Starting with custom patterns...${NC}"
                echo "2" | "$BINARY"
                ;;
            3)
                echo -e "${YELLOW}运行性能测试... Running benchmark...${NC}"
                echo "3" | "$BINARY"
                ;;
            4)
                echo -e "${YELLOW}高级设置... Advanced settings...${NC}"
                echo "4" | "$BINARY"
                ;;
            5)
                view_results
                ;;
            0)
                echo -e "${GREEN}感谢使用！Goodbye!${NC}"
                exit 0
                ;;
            *)
                echo -e "${RED}❌ 无效选择 | Invalid choice${NC}"
                ;;
        esac
        
        show_stats
    done
}

# 主程序开始
main() {
    show_banner
    
    # 检查Rust和Cargo
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ 错误: 找不到 Cargo${NC}"
        echo "Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    check_and_compile
    
    echo -e "${GREEN}✓ 准备完成！Ready to go!${NC}"
    echo ""
    echo -e "${YELLOW}二进制文件: ${GREEN}$BINARY${NC}"
    echo -e "${YELLOW}输出文件: ${GREEN}$OUTPUT_FILE${NC}"
    echo ""
    
    # 如果提供了参数，直接运行对应操作
    if [ $# -gt 0 ]; then
        case "$1" in
            --run)
                "$BINARY"
                ;;
            --benchmark)
                echo "3" | "$BINARY"
                ;;
            --view)
                view_results
                ;;
            *)
                main_loop
                ;;
        esac
    else
        main_loop
    fi
}

# 运行主程序
main "$@"

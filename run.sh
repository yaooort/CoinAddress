#!/bin/bash

# TRON 波场靓号生成器 - 使用脚本示例

# 编译项目
echo "编译项目... Building project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "编译失败！Compilation failed!"
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║  TRON 波场靓号生成器已准备就绪！                          ║"
echo "║  TRON Vanity Address Generator is ready!                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "使用方法 Usage: ./target/release/tron-vanity"
echo ""
echo "快速开始 Quick Start:"
echo "  1. 选择 '1' 使用默认靓号模式"
echo "  2. 选择 '2' 自定义靓号模式"
echo "  3. 选择 '3' 运行性能测试"
echo "  4. 选择 '4' 高级设置"
echo ""
echo "结果将保存到: Results saved to: tron_vanity.txt"
echo ""

# 运行程序
./target/release/tron-vanity

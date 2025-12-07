#!/bin/bash

# GitHub Actions 部署脚本
# 快速设置和推送到 GitHub

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║     GitHub Actions 快速部署脚本                              ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 检查 Git 安装
if ! command -v git &> /dev/null; then
    echo -e "${RED}✗ 错误: Git 未安装${NC}"
    echo "  请先安装 Git: https://git-scm.com/downloads"
    exit 1
fi

echo -e "${GREEN}✓ Git 已安装${NC}"
echo ""

# 检查是否已初始化 Git
if [ ! -d ".git" ]; then
    echo -e "${YELLOW}⚠ Git 仓库未初始化${NC}"
    echo ""
    echo "初始化 Git 仓库..."
    git init
    echo -e "${GREEN}✓ Git 仓库已初始化${NC}"
else
    echo -e "${GREEN}✓ Git 仓库已存在${NC}"
fi

echo ""

# 检查 remote 是否配置
if git config --get remote.origin.url &> /dev/null; then
    REMOTE_URL=$(git config --get remote.origin.url)
    echo -e "${GREEN}✓ 远程仓库已配置:${NC} $REMOTE_URL"
else
    echo -e "${YELLOW}⚠ 远程仓库未配置${NC}"
    echo ""
    read -p "请输入 GitHub 仓库 URL (https://github.com/...): " REMOTE_URL
    
    if [ -z "$REMOTE_URL" ]; then
        echo -e "${RED}✗ 仓库 URL 不能为空${NC}"
        exit 1
    fi
    
    git remote add origin "$REMOTE_URL"
    echo -e "${GREEN}✓ 远程仓库已添加${NC}"
fi

echo ""

# 检查 Git 用户配置
if ! git config --global user.email &> /dev/null; then
    echo -e "${YELLOW}⚠ Git 用户未配置${NC}"
    read -p "请输入 Git 用户名: " GIT_USER
    read -p "请输入 Git 邮箱: " GIT_EMAIL
    
    git config --global user.name "$GIT_USER"
    git config --global user.email "$GIT_EMAIL"
    echo -e "${GREEN}✓ Git 用户已配置${NC}"
else
    GIT_USER=$(git config --global user.name)
    echo -e "${GREEN}✓ Git 用户: $GIT_USER${NC}"
fi

echo ""

# 检查工作流文件
echo "检查工作流文件..."
WORKFLOW_FILES=(
    ".github/workflows/release.yml"
    ".github/workflows/ci.yml"
)

for file in "${WORKFLOW_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ $file${NC}"
    else
        echo -e "${RED}✗ $file 未找到${NC}"
        exit 1
    fi
done

echo ""

# 显示当前状态
echo "📊 当前状态:"
echo ""
git status --short | head -10
echo ""

# 确认提交
read -p "是否提交所有更改? (y/n): " CONFIRM_COMMIT

if [ "$CONFIRM_COMMIT" = "y" ] || [ "$CONFIRM_COMMIT" = "Y" ]; then
    echo ""
    echo "提交文件..."
    git add .
    git commit -m "Add GitHub Actions workflows for automated release" || echo -e "${YELLOW}⚠ 无新文件提交${NC}"
    echo -e "${GREEN}✓ 提交完成${NC}"
else
    echo -e "${YELLOW}⚠ 已跳过提交${NC}"
fi

echo ""

# 推送到 main 分支
echo "推送到 GitHub..."
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${YELLOW}当前分支: $CURRENT_BRANCH${NC}"
    read -p "是否切换到 main 分支并推送? (y/n): " CONFIRM_BRANCH
    
    if [ "$CONFIRM_BRANCH" = "y" ] || [ "$CONFIRM_BRANCH" = "Y" ]; then
        git branch -M main
        git push -u origin main
    fi
else
    git push origin main
fi

echo -e "${GREEN}✓ 代码已推送到 GitHub${NC}"
echo ""

# 创建 Release
read -p "现在创建 Release? (y/n): " CONFIRM_RELEASE

if [ "$CONFIRM_RELEASE" = "y" ] || [ "$CONFIRM_RELEASE" = "Y" ]; then
    echo ""
    echo "创建 Release..."
    
    read -p "请输入版本号 (例如: v0.2.0): " VERSION
    
    if [ -z "$VERSION" ]; then
        echo -e "${RED}✗ 版本号不能为空${NC}"
        exit 1
    fi
    
    # 验证版本号格式
    if ! [[ $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo -e "${YELLOW}⚠ 警告: 版本号不符合语义化版本格式 (v主.次.补丁)${NC}"
        read -p "是否继续? (y/n): " CONFIRM_VERSION
        if [ "$CONFIRM_VERSION" != "y" ] && [ "$CONFIRM_VERSION" != "Y" ]; then
            exit 1
        fi
    fi
    
    read -p "请输入 Release 说明 (默认: Release $VERSION): " RELEASE_MSG
    if [ -z "$RELEASE_MSG" ]; then
        RELEASE_MSG="Release $VERSION"
    fi
    
    echo ""
    echo "创建 tag..."
    git tag -a "$VERSION" -m "$RELEASE_MSG"
    
    echo "推送 tag..."
    git push origin "$VERSION"
    
    echo -e "${GREEN}✓ Release 已创建: $VERSION${NC}"
    echo ""
    echo "📢 GitHub Actions 正在编译..."
    echo "请访问查看编译进度:"
    echo ""
    
    REPO_URL=$(git config --get remote.origin.url)
    REPO_NAME=$(echo "$REPO_URL" | sed 's/.*\/\([^/]*\)\.git$/\1/')
    REPO_OWNER=$(echo "$REPO_URL" | sed 's/.*[:/]\([^/]*\)\/.*$/\1/')
    
    echo "  🔗 Actions: https://github.com/$REPO_OWNER/$REPO_NAME/actions"
    echo "  🔗 Release: https://github.com/$REPO_OWNER/$REPO_NAME/releases/tag/$VERSION"
else
    echo -e "${YELLOW}⚠ 已跳过 Release 创建${NC}"
    echo ""
    echo "稍后创建 Release:"
    echo ""
    echo "  # 创建 tag"
    echo "  git tag -a v0.2.0 -m \"Release 0.2.0\""
    echo ""
    echo "  # 推送 tag"
    echo "  git push origin v0.2.0"
fi

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo -e "${GREEN}✨ 部署完成！${NC}"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "📚 相关文档:"
echo "  • GITHUB-ACTIONS-GUIDE.md - 完整指南"
echo "  • GITHUB-ACTIONS-CHECKLIST.md - 检查清单"
echo ""

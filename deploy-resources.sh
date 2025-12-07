#!/bin/bash

# 部署脚本 - 为各平台配置Logo和资源

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
LOGO_DIR="$SCRIPT_DIR/assets/logos"
APP_NAME="TRON Vanity"

echo "📦 部署应用资源..."

# macOS应用包配置
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 配置 macOS 资源..."
    
    # 创建应用包结构
    APP_BUNDLE="$SCRIPT_DIR/target/release/TRON\ Vanity.app"
    mkdir -p "$APP_BUNDLE/Contents/"{MacOS,Resources}
    
    # 生成Info.plist
    cat > "$APP_BUNDLE/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>zh_CN</string>
    <key>CFBundleExecutable</key>
    <string>tron-vanity</string>
    <key>CFBundleIdentifier</key>
    <string>com.tronvanity.app</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>TRON Vanity</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.2.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHumanReadableCopyright</key>
    <string>© 2024 TRON Vanity. All rights reserved.</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
EOF
    
    # 复制Logo到Resources
    if [ -f "$LOGO_DIR/icon.svg" ]; then
        cp "$LOGO_DIR/icon.svg" "$APP_BUNDLE/Contents/Resources/"
        echo "✓ macOS资源已配置"
    fi
fi

# Windows资源配置
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    echo "🪟 配置 Windows 资源..."
    
    # 创建快捷方式配置
    cat > "$SCRIPT_DIR/tron-vanity.lnk.bat" << 'EOF'
@echo off
REM 创建桌面快捷方式
set DESKTOP=%USERPROFILE%\Desktop
set SCRIPT_DIR=%~dp0

powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%DESKTOP%\TRON Vanity.lnk'); $Shortcut.TargetPath = '%SCRIPT_DIR%tron-vanity.exe'; $Shortcut.Save()"

echo 快捷方式已创建到桌面
EOF
    
    echo "✓ Windows资源已配置"
fi

# Linux桌面整合
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🐧 配置 Linux 资源..."
    
    # 创建.desktop文件
    mkdir -p ~/.local/share/applications
    
    cat > ~/.local/share/applications/tron-vanity.desktop << EOF
[Desktop Entry]
Type=Application
Name=TRON Vanity
Comment=Professional TRON Address Generator
Exec=$SCRIPT_DIR/target/release/tron-vanity
Icon=$LOGO_DIR/icon.svg
Categories=Utility;
Terminal=false
EOF
    
    echo "✓ Linux资源已配置"
fi

echo ""
echo "✨ 应用资源部署完成！"
echo ""
echo "📍 Logo位置: $LOGO_DIR"
ls -lh "$LOGO_DIR" 2>/dev/null || echo "  Logo目录未找到"

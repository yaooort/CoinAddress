#!/usr/bin/env python3
"""
TRON Vanity Logo 生成器
生成专业的品牌Logo，支持多种格式
"""

import os
import sys
from pathlib import Path

def generate_svg_logo():
    """生成主Logo SVG"""
    svg_content = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="512" height="512" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
  <!-- 背景 -->
  <defs>
    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#ff6b35;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#f7931a;stop-opacity:1" />
    </linearGradient>
    <linearGradient id="grad2" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#40d4ff;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#00bfff;stop-opacity:1" />
    </linearGradient>
    <filter id="shadow">
      <feDropShadow dx="0" dy="4" stdDeviation="8" flood-opacity="0.3"/>
    </filter>
  </defs>

  <!-- 背景圆 -->
  <circle cx="256" cy="256" r="240" fill="url(#grad1)" opacity="0.15"/>
  
  <!-- 外圆环 -->
  <circle cx="256" cy="256" r="230" fill="none" stroke="url(#grad1)" stroke-width="3" opacity="0.8"/>
  
  <!-- 中心几何形状 - 闪闪发光的钻石 -->
  <g filter="url(#shadow)">
    <!-- 外菱形 -->
    <polygon points="256,80 400,256 256,432 112,256" fill="url(#grad1)" opacity="0.9"/>
    
    <!-- 内菱形 -->
    <polygon points="256,140 340,256 256,372 172,256" fill="url(#grad2)" opacity="0.85"/>
    
    <!-- 中心星形 -->
    <polygon points="256,180 280,240 340,260 280,280 256,340 232,280 172,260 232,240" 
             fill="#ffffff" opacity="0.95"/>
  </g>
  
  <!-- 闪光效果 -->
  <circle cx="220" cy="200" r="8" fill="#ffffff" opacity="0.7">
    <animate attributeName="opacity" values="0.7;0.3;0.7" dur="2s" repeatCount="indefinite"/>
  </circle>
  <circle cx="290" cy="220" r="6" fill="#ffffff" opacity="0.5">
    <animate attributeName="opacity" values="0.5;0.2;0.5" dur="2.5s" repeatCount="indefinite"/>
  </circle>
  
  <!-- 环形动画 -->
  <circle cx="256" cy="256" r="200" fill="none" stroke="url(#grad2)" stroke-width="2" 
          opacity="0.3" stroke-dasharray="5,5">
    <animateTransform attributeName="transform" type="rotate" 
                      from="0 256 256" to="360 256 256" dur="20s" repeatCount="indefinite"/>
  </circle>
  
  <!-- 文字 -->
  <text x="256" y="450" font-family="Arial, sans-serif" font-size="36" font-weight="bold" 
        text-anchor="middle" fill="url(#grad1)">
    TRON VANITY
  </text>
  
  <!-- 副标题 -->
  <text x="256" y="480" font-family="Arial, sans-serif" font-size="12" 
        text-anchor="middle" fill="#666666" letter-spacing="2">
    ADDRESS GENERATOR
  </text>
</svg>'''
    return svg_content


def generate_icon_svg():
    """生成应用图标 SVG (简化版)"""
    svg_content = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="256" height="256" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="iconGrad1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#ff6b35;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#f7931a;stop-opacity:1" />
    </linearGradient>
    <linearGradient id="iconGrad2" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#40d4ff;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#00bfff;stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <!-- 背景 -->
  <rect width="256" height="256" fill="#f0f0f0" rx="50"/>
  
  <!-- 主菱形 -->
  <polygon points="128,40 200,128 128,216 56,128" fill="url(#iconGrad1)" opacity="0.9"/>
  
  <!-- 内菱形 -->
  <polygon points="128,80 160,128 128,176 96,128" fill="url(#iconGrad2)" opacity="0.85"/>
  
  <!-- 星形中心 -->
  <polygon points="128,100 140,125 165,135 140,145 128,170 116,145 91,135 116,125" 
           fill="#ffffff" opacity="0.9"/>
</svg>'''
    return svg_content


def generate_banner_svg():
    """生成横幅 SVG"""
    svg_content = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="1200" height="400" viewBox="0 0 1200 400" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bannerGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#ff6b35;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#40d4ff;stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <!-- 背景 -->
  <rect width="1200" height="400" fill="#f9f9f9"/>
  
  <!-- 渐变背景 -->
  <rect width="1200" height="400" fill="url(#bannerGrad)" opacity="0.1"/>
  
  <!-- 左侧Logo -->
  <g transform="translate(100, 50)">
    <polygon points="128,40 200,128 128,216 56,128" fill="#ff6b35" opacity="0.9"/>
    <polygon points="128,80 160,128 128,176 96,128" fill="#40d4ff" opacity="0.85"/>
  </g>
  
  <!-- 文字内容 -->
  <text x="350" y="120" font-family="Arial, sans-serif" font-size="64" font-weight="bold" 
        fill="#ff6b35">
    TRON VANITY
  </text>
  
  <text x="350" y="180" font-family="Arial, sans-serif" font-size="24" fill="#666">
    Professional TRON Address Generator
  </text>
  
  <text x="350" y="250" font-family="Arial, sans-serif" font-size="16" fill="#999">
    Generate beautiful vanity addresses with real-time monitoring
  </text>
  
  <!-- 装饰线 -->
  <line x1="350" y1="270" x2="1100" y2="270" stroke="#ff6b35" stroke-width="2"/>
</svg>'''
    return svg_content


def generate_favicon_ico():
    """生成Favicon ICO（使用SVG转换）"""
    svg_content = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="64" height="64" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="faviconGrad1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#ff6b35;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#f7931a;stop-opacity:1" />
    </linearGradient>
    <linearGradient id="faviconGrad2" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#40d4ff;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#00bfff;stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <rect width="256" height="256" fill="#ffffff"/>
  <polygon points="128,30 210,128 128,226 46,128" fill="url(#faviconGrad1)"/>
  <polygon points="128,80 170,128 128,176 86,128" fill="url(#faviconGrad2)"/>
  <circle cx="128" cy="128" r="15" fill="#ffffff"/>
</svg>'''
    return svg_content


def main():
    output_dir = Path("assets/logos")
    output_dir.mkdir(parents=True, exist_ok=True)
    
    print("📝 生成 TRON Vanity Logo...")
    print()
    
    # 生成主Logo
    logo_file = output_dir / "logo.svg"
    with open(logo_file, 'w') as f:
        f.write(generate_svg_logo())
    print(f"✓ 主Logo已生成: {logo_file}")
    
    # 生成应用图标
    icon_file = output_dir / "icon.svg"
    with open(icon_file, 'w') as f:
        f.write(generate_icon_svg())
    print(f"✓ 应用图标已生成: {icon_file}")
    
    # 生成横幅
    banner_file = output_dir / "banner.svg"
    with open(banner_file, 'w') as f:
        f.write(generate_banner_svg())
    print(f"✓ 横幅已生成: {banner_file}")
    
    # 生成Favicon
    favicon_file = output_dir / "favicon.svg"
    with open(favicon_file, 'w') as f:
        f.write(generate_favicon_ico())
    print(f"✓ Favicon已生成: {favicon_file}")
    
    print()
    print("🎨 颜色方案:")
    print("  • 主色 (Orange):  #ff6b35 / RGB(255, 107, 53)")
    print("  • 副色 (Cyan):    #40d4ff / RGB(64, 212, 255)")
    print("  • 强调 (Gold):    #f7931a / RGB(247, 147, 26)")
    print()
    print("📁 输出目录: ", output_dir)
    print()
    print("✨ 所有Logo文件生成完成！")
    print()
    print("💡 建议用途:")
    print("  • logo.svg        - 网站、文档、社交媒体")
    print("  • icon.svg        - 应用图标、桌面快捷方式")
    print("  • banner.svg      - GitHub README、营销资料")
    print("  • favicon.svg     - 网站标签栏、浏览器书签")
    

if __name__ == "__main__":
    main()

# 打包指南

## macOS 打包

在 macOS 上运行打包脚本：

```bash
bash build.sh
```

这会生成：

- `tron-vanity_0.2.0_macos.dmg` - macOS DMG 安装包

**包含内容：**

- ✅ 可执行程序（tron-vanity.app）
- ✅ 自动使用 PingFang SC 中文字体
- ✅ 可直接双击运行

**安装方法：**

1. 双击 DMG 文件
2. 将应用拖入 Applications 文件夹
3. 从 Launchpad 或应用程序文件夹启动

---

## Windows 打包

### 方法 1：在 Windows CMD 中运行批处理脚本

```cmd
build.bat
```

### 方法 2：在 PowerShell 中运行

```powershell
powershell -ExecutionPolicy Bypass -File build.bat
```

这会生成：

- `tron-vanity_0.2.0_windows.zip` - Windows 可执行包

**包含内容：**

- ✅ tron-vanity.exe 可执行文件
- ✅ 运行.bat 启动脚本
- ✅ 自动使用 Microsoft YaHei 中文字体
- ✅ 完整文档

**安装方法：**

1. 解压 ZIP 文件
2. 双击 `运行.bat` 启动程序
3. 或直接双击 `tron-vanity.exe`

---

## 系统需求

### macOS

- macOS 10.13+
- 自带 PingFang SC 字体（无需额外安装）

### Windows

- Windows 7 SP1 或更新
- 建议安装 Visual C++ Redistributable（如果遇到缺失 DLL）
- 自带 Microsoft YaHei 字体（Windows XP+）

---

## 故障排除

### macOS 中文显示问题

如果中文不显示：

1. 确保系统字体完整
2. 尝试在终端运行：`./target/release/tron-vanity`

### Windows 中文显示问题

如果中文不显示：

1. 确保系统已安装 Microsoft YaHei 字体（Windows 自带）
2. 尝试在 PowerShell 中运行程序
3. 检查代码页设置：`chcp 65001` 后重试

### 程序运行缓慢

- 减少线程数（降低 CPU 占用）
- 增加批处理大小以提高效率
- 确保系统有足够的可用内存

---

## 开发者构建

如果需要从源代码编译：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆仓库
git clone <repo-url>
cd CoinAddress

# 构建 Release 版本
cargo build --release --bin tron-vanity

# 运行
./target/release/tron-vanity
```

---

## 版本信息

- **当前版本**：0.2.0
- **支持平台**：macOS (Intel/Apple Silicon), Windows, Linux
- **中文字体**：
  - macOS: PingFang SC
  - Windows: Microsoft YaHei
  - Linux: WenQuanYi Micro Hei

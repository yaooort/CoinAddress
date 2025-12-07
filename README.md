# 多链靓号生成器 | TRON / EVM / SOL Vanity

Rust 实现的多链靓号地址生成器，支持 TRON、EVM（以太坊兼容）、Solana，输出地址 / 公钥 / 私钥 / BIP39 助记词，含 GUI 与 CLI。

## 下载与更新

- 点击下方按钮跳转到 GitHub Releases，获取已编译的自动更新包（DMG / DEB / MSI）。

[![Download Latest Release](https://img.shields.io/github/v/release/yaooort/CoinAddress?label=%E4%B8%8B%E8%BD%BD%E6%9C%80%E6%96%B0%E7%89%88&logo=github)](https://github.com/yaooort/CoinAddress/releases/latest)

## 功能特性 ✨

- **多链支持**：TRON（Base58check / 0x41 前缀）、EVM（0x 开头 keccak 后 20 字节）、Solana（ed25519 / Base58）
- **GPU-free 高性能**：CPU 多线程并行生成，实时速率统计
- **现代 GUI（iced）**：暗色主题、链选择器、实时日志、CPU/内存仪表盘
- **靓号规则**：
  - 末尾匹配自定义模式（逗号分隔）
  - 末尾连续相同字符 ≥3 个（默认规则）
- **输出与保存**：
  - 自动把发现的靓号追加到对应链文件：`tron_vanity.txt` / `evm_vanity.txt` / `sol_vanity.txt`
  - “保存当前靓号”按钮可选择任意路径单独导出
- **CI/CD**：GitHub Actions 自动为多平台编译并附加到 Release（含 macOS x86_64/ARM64）

## 系统要求

- **操作系统**: macOS, Linux, Windows
- **Rust**: 1.70+
- **内存**: 建议 4GB+
- **CPU**: 多核处理器效果最佳

## 编译

### Release 模式（GUI）

```bash
cargo build --release --bin tron-vanity
```

### CLI 构建

```bash
cargo build --release --bin tron-vanity-cli
```

## 使用

### GUI 快速开始

```bash
./target/release/tron-vanity
```

操作步骤：

1. 选择链：TRON / EVM / SOL
2. 输入末尾匹配模式（逗号分隔，可留空使用默认规则）
3. 设置批处理大小、线程数
4. 点“启动”，可随时“暂停”/“停止”
5. 发现靓号会写入对应链的 `*_vanity.txt`；点“保存当前靓号”可自选路径导出

### CLI（可选）

```bash
./target/release/tron-vanity-cli
```

按提示选择默认/自定义模式或性能测试。

## 输出格式

### 控制台输出（靓号）

```
╔════════════════════════════════════════════════════════════╗
║ 发现靓号! | Found Vanity Address! |
╠════════════════════════════════════════════════════════════╣
地址 | Address: TAaa1wWwEM4zg3nZ6bWfYwXeQwv4pEiTFj
私钥 | Private Key: 5a488d916d9f59803df680d2c65a6a3b7a65a78164731531db42293900825bec
公钥 | Public Key: 04a006ee45ffa396efb94c80040b6d688e8be04eee68b4492b350b8ed8791f...
助记词 | Mnemonic: fog dutch gold swamp void scale water source spot crazy once jealous
╚════════════════════════════════════════════════════════════╝
```

### 文件输出 (链分文件)

```
═══════════════════════════════════════════════════════════
[VANITY] 2025-12-07 10:30:45
Address: TAaa1wWwEM4zg3nZ6bWfYwXeQwv4pEiTFj
Private Key: 5a488d916d9f59803df680d2c65a6a3b7a65a78164731531db42293900825bec
Public Key: 04a006ee45ffa396efb94c80040b6d688e8be04eee68b4492b350b8ed8791f...
Mnemonic: fog dutch gold swamp void scale water source spot crazy once jealous
═══════════════════════════════════════════════════════════
```

TRON / EVM / SOL 会分别写入 `tron_vanity.txt`、`evm_vanity.txt`、`sol_vanity.txt`。

## 靓号检测规则

程序会自动识别以下类型的靓号：

### 1. 末尾连续相同字符（≥3 个）

- 尾部满足 `...aaa` / `...111` / `...BBB` 等

### 2. 自定义末尾模式

- 逗号分隔输入的任意后缀，例如 `8888, luck, 0000`

## 性能指标

在标准硬件上的预期性能（Release 模式）：

| 场景           | 速率              | 备注        |
| -------------- | ----------------- | ----------- |
| 单线程         | ~100-150 addr/s   | 视 CPU 而定 |
| 多线程 (8 核)  | ~800-1200 addr/s  | 视 CPU 而定 |
| 多线程 (16 核) | ~1500-2200 addr/s | 视 CPU 而定 |

**靓号概率**:

- 靓号（3 个连续相同字符）: 约 1/46656
- 需要生成约 50000 个地址才能找到一个靓号

## 文件说明

### Cargo.toml

项目配置，包含所有依赖库：

- `sha2`, `ripemd`: 哈希算法
- `k256`: Secp256k1 曲线密码学
- `bip39`: BIP39 助记词生成
- `bs58`: Base58 编码
- `rayon`: 多线程并行处理
- `colored`: 彩色输出

### src/lib.rs

核心库函数：

- `generate_vanity_address(chain)`: 生成 TRON / EVM / SOL 地址
- `is_vanity_address()`: 末尾模式 / 末尾连续字符检测
- `save_address_to_file()`: 按链写入文本

### src/gui.rs

基于 iced 的 GUI：链选择、仪表盘、日志、手动保存按钮。

### src/cli.rs

简易交互式 CLI（默认 / 自定义模式 / 基准）。

## TRON 地址生成算法

1. **生成私钥**: 随机 32 字节
2. **生成公钥**: 使用 Secp256k1 曲线从私钥推导公钥
3. **地址生成**:
   - SHA256(公钥)
   - RIPEMD160(结果)
   - 添加版本字节 (0x41 for TRON)
   - 计算校验和 (前 4 字节的 SHA256(SHA256))
   - Base58 编码
4. **生成助记词**: 使用前 16 字节私钥生成 12 个单词

## 常见问题

### Q: 如何确保生成的地址有效？

A: 所有地址都使用标准 TRON 算法生成，并包含校验和验证。可以在 TRON 浏览器验证。

### Q: 找到靓号需要多久？

A: 取决于硬件和靓号复杂度。3 个连续字符的靓号平均需要 5-10 分钟（8 核 CPU）。

### Q: 生成的私钥是否安全？

A: 使用系统随机数生成器。对于生产用途，建议在离线环境中使用。

### Q: 可以用 GPU 加速吗？

A: 当前版本使用 CPU 多线程。GPU 加速可作为未来优化项。

### Q: 如何导入生成的地址到钱包？

A: 使用私钥（Hex 格式）或助记词（BIP39 格式）导入到支持 TRON 的钱包。

## 安全建议

⚠️ **重要**:

- 仅在可信的离线环境中使用此工具
- 生成的私钥不要与任何人共享
- 不要在公共网络上运行
- 自己编译源代码以确保安全性

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### v0.2.2 (2025-12-07)

- 支持 TRON / EVM / SOL 多链生成
- GUI 重构：链选择、仪表盘、手动保存当前靓号
- Release 工作流支持 macOS x86_64 / ARM64 并行构建

### v0.1.0 (2025-12-07)

- 初始版本发布（TRON 单链、CLI）

## 联系方式

如有问题或建议，欢迎反馈！

---

**免责声明**: 本工具仅供学习和研究使用。使用者对其产生的任何后果负全责。

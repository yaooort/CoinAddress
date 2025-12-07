# 使用示例 | Usage Examples

## 基本使用 | Basic Usage

### 1. 使用默认靓号模式

```bash
$ ./target/release/tron-vanity
```

选择选项 1 后，程序会使用预设的靓号模式：

- 数字: 0000, 1111, 2222, ..., 9999
- 字母: AAAA, BBBB, CCCC, DDDD

输出示例：

```
╔════════════════════════════════════════════════════════════╗
║ 发现靓号! | Found Vanity Address! |
╠════════════════════════════════════════════════════════════╣
地址 | Address: TAaa1wWwEM4zg3nZ6bWfYwXeQwv4pEiTFj
私钥 | Private Key: 5a488d916d9f59803df680d2c65a6a3b7a65a78164731531db42293900825bec
公钥 | Public Key: 04a006ee45ffa396efb94c80040b6d688e8be04eee68b4492b350b8ed8791f5a...
助记词 | Mnemonic: fog dutch gold swamp void scale water source spot crazy once jealous
╚════════════════════════════════════════════════════════════╝
```

### 2. 自定义靓号模式

```bash
$ ./target/release/tron-vanity
```

选择选项 2，然后输入你想要的模式（逗号分隔）：

```
输入想要的靓号模式 (逗号分隔，如: 1111,AAAA,好运): Lucky,888,666
```

程序会搜索包含这些子串的地址：

- 包含 "Lucky"
- 包含 "888"
- 包含 "666"

### 3. 性能测试

```bash
$ ./target/release/tron-vanity
```

选择选项 3 可以运行完整的性能基准测试：

```
► 单线程性能测试 (Single-threaded benchmark)...
✓ 100 个地址生成耗时: 1.05s | 速率: 95.24 addr/s

► 多线程性能测试 (Multi-threaded benchmark)...
✓ 1000 个地址生成耗时: 1.23s | 速率: 813.01 addr/s

► 超大规模测试 (Large scale benchmark)...
✓ 10000 个地址生成耗时: 12.45s | 速率: 803.21 addr/s
```

### 4. 高级设置

```bash
$ ./target/release/tron-vanity
```

选择选项 4，自定义以下参数：

```
保存所有生成的地址? (Save all addresses? y/n): y
线程数 (Number of threads, default 8): 16
批处理大小 (Batch size, default 1000): 2000
```

## 输出文件

### 文件格式

所有靓号保存在 `tron_vanity.txt`：

```
═══════════════════════════════════════════════════════════
[VANITY] 2025-12-07 10:30:45
Address: TAaa1wWwEM4zg3nZ6bWfYwXeQwv4pEiTFj
Private Key: 5a488d916d9f59803df680d2c65a6a3b7a65a78164731531db42293900825bec
Public Key: 04a006ee45ffa396efb94c80040b6d688e8be04eee68b4492b350b8ed8791f5a...
Mnemonic: fog dutch gold swamp void scale water source spot crazy once jealous
═══════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════
[VANITY] 2025-12-07 10:31:12
Address: TB1111BBBvZbriaNxCuEJtJnTWuuUcavNP
...
```

### 查看靓号

```bash
# 查看所有找到的靓号
grep -B3 "Address:" tron_vanity.txt

# 仅显示地址
grep "^Address:" tron_vanity.txt

# 计数靓号个数
grep -c "^\[VANITY\]" tron_vanity.txt
```

## 批量导入钱包

### 方法 1: 使用私钥导入

从 `tron_vanity.txt` 中提取私钥：

```bash
# 提取所有私钥
grep "^Private Key:" tron_vanity.txt | awk '{print $3}' > private_keys.txt
```

然后在钱包中导入每个私钥（TRON 钱包 → 导入账户 → 选择"私钥"）

### 方法 2: 使用助记词导入

```bash
# 提取所有助记词
grep "^Mnemonic:" tron_vanity.txt | cut -d':' -f2- > mnemonics.txt
```

在钱包中使用助记词恢复账户（TRON 钱包 → 恢复钱包 → 输入助记词）

## 优化建议

### 1. 对于寻找特定模式

```
输入靓号模式: Tron,Chain,Web3,Crypto
```

这会加快特定模式的搜索速度。

### 2. 对于需要大量靓号

选择选项 4，增加线程数和批处理大小：

```
线程数: 32
批处理大小: 5000
保存所有地址: y
```

### 3. 对于运行多个实例

在不同的窗口运行多个程序实例，每个使用不同的输出文件：

```bash
# 终端 1
./target/release/tron-vanity > results_1.txt 2>&1

# 终端 2
./target/release/tron-vanity > results_2.txt 2>&1

# 终端 3
./target/release/tron-vanity > results_3.txt 2>&1
```

## 故障排除 | Troubleshooting

### Q: 程序运行很慢

A: 检查系统的可用 CPU 核心数。如果只有 2-4 核，考虑：

- 减少线程数
- 减少批处理大小
- 关闭其他应用

### Q: 内存占用过高

A: 减少批处理大小：

```
批处理大小: 500
```

### Q: 找不到指定的靓号

A: 某些模式可能需要很长时间。例如：

- 4 个连续字符的概率: ~1/1679616
- 5 个连续字符的概率: ~1/60466176

对于非常罕见的模式，可能需要运行数小时甚至数天。

### Q: 如何验证生成的地址有效性

A: 访问 TRON 浏览器：https://tronscan.org

在搜索框输入生成的地址，应该能查到该地址的详细信息。

## 安全建议 ⚠️

1. **离线运行**: 在断网或离线计算机上使用此工具
2. **备份私钥**: 找到好的靓号后，立即保存私钥副本
3. **不要共享**: 绝不要与任何人分享私钥或助记词
4. **源代码审计**: 自行编译源代码以确保安全
5. **小额测试**: 导入钱包前，先用小额 TRON 测试

## 相关资源

- TRON 浏览器: https://tronscan.org
- TRON 官网: https://tron.network
- TRON 文档: https://developers.tron.network

## 许可证

MIT License - 可自由使用和修改

---

**免责声明**: 本工具仅供学习和研究使用。不对任何因使用本工具而造成的损失负责。

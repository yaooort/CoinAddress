use std::{fs::OpenOptions, io::Write};

use chrono::Local;
use ed25519_dalek::{PublicKey, SecretKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand::RngCore;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};

pub mod monitor;

/// 支持的链类型
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChainType {
    Tron,
    Evm,
    Sol,
}

impl ChainType {
    pub fn label(self) -> &'static str {
        match self {
            ChainType::Tron => "TRON",
            ChainType::Evm => "EVM",
            ChainType::Sol => "SOL",
        }
    }
}

impl std::fmt::Display for ChainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// 通用的靓号结果结构
#[derive(Clone, Debug)]
pub struct VanityAddress {
    pub chain: ChainType,
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub mnemonic: String,
}

/// 生成随机 secp256k1 私钥
pub fn generate_private_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

/// 从私钥生成 secp256k1 公钥（未压缩）
pub fn private_key_to_public_key(private_key: &[u8; 32]) -> Vec<u8> {
    use k256::SecretKey;
    let secret = SecretKey::from_slice(private_key).expect("valid key");
    secret
        .public_key()
        .to_encoded_point(false)
        .as_bytes()
        .to_vec()
}

/// 从公钥生成 TRON 地址
pub fn public_key_to_tron_address(public_key: &[u8]) -> String {
    // 1. SHA256 公钥（跳过前缀字节 0x04）
    let mut hasher = Sha256::new();
    hasher.update(&public_key[1..]);
    let sha256_hash = hasher.finalize();

    // 2. RIPEMD160
    let mut hasher = Ripemd160::new();
    hasher.update(&sha256_hash);
    let ripemd160_hash = hasher.finalize();

    // 3. 添加版本字节 0x41
    let mut versioned = vec![0x41];
    versioned.extend_from_slice(&ripemd160_hash);

    // 4. 双 SHA256 取前 4 字节做校验和
    let mut hasher = Sha256::new();
    hasher.update(&versioned);
    let sha256_1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&sha256_1);
    let sha256_2 = hasher.finalize();

    let checksum = &sha256_2[0..4];

    // 5. 拼接并 Base58 编码
    let mut address_bytes = versioned;
    address_bytes.extend_from_slice(checksum);

    bs58::encode(&address_bytes).into_string()
}

/// 生成 TRON 地址
pub fn generate_tron_address() -> VanityAddress {
    let private_key = generate_private_key();
    let public_key = private_key_to_public_key(&private_key);

    let address = public_key_to_tron_address(&public_key);
    let mnemonic = generate_mnemonic(&private_key);

    VanityAddress {
        chain: ChainType::Tron,
        address,
        public_key: hex::encode(&public_key),
        private_key: hex::encode(&private_key),
        mnemonic,
    }
}

/// 生成 EVM 地址（以太坊兼容）
pub fn generate_evm_address() -> VanityAddress {
    let private_key = generate_private_key();
    let public_key = private_key_to_public_key(&private_key);

    // keccak256 公钥（去掉 0x04 前缀）后取后 20 字节
    let mut keccak = Keccak::v256();
    keccak.update(&public_key[1..]);
    let mut out = [0u8; 32];
    keccak.finalize(&mut out);
    let address_bytes = &out[12..];
    let address = format!("0x{}", hex::encode(address_bytes));

    let mnemonic = generate_mnemonic(&private_key);

    VanityAddress {
        chain: ChainType::Evm,
        address,
        public_key: hex::encode(&public_key),
        private_key: hex::encode(&private_key),
        mnemonic,
    }
}

/// 生成 Solana 地址
pub fn generate_sol_address() -> VanityAddress {
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let secret = SecretKey::from_bytes(&seed).expect("valid seed");
    let public: PublicKey = (&secret).into();

    let address = bs58::encode(public.as_bytes()).into_string();
    let mnemonic = generate_mnemonic(&seed);

    VanityAddress {
        chain: ChainType::Sol,
        address,
        public_key: hex::encode(public.as_bytes()),
        private_key: hex::encode(seed),
        mnemonic,
    }
}

/// 按链类型生成地址
pub fn generate_vanity_address(chain: ChainType) -> VanityAddress {
    match chain {
        ChainType::Tron => generate_tron_address(),
        ChainType::Evm => generate_evm_address(),
        ChainType::Sol => generate_sol_address(),
    }
}

/// 生成 BIP39 助记词（12 词）
fn generate_mnemonic(entropy: &[u8]) -> String {
    use bip39::Mnemonic;
    let entropy_slice = &entropy[0..16.min(entropy.len())];
    match Mnemonic::from_entropy(entropy_slice) {
        Ok(m) => m.to_string(),
        Err(_) => "unable to generate mnemonic".to_string(),
    }
}

/// 检查是否为靓号：末尾匹配模式或末尾连续 >=3 相同字符
pub fn is_vanity_address(address: &str, patterns: &[&str]) -> bool {
    let address_lower = address.to_lowercase();

    if !patterns.is_empty() {
        for pattern in patterns {
            if address_lower.ends_with(&pattern.to_lowercase()) {
                return true;
            }
        }
        return false;
    }

    let chars: Vec<char> = address_lower.chars().collect();
    if chars.len() < 3 {
        return false;
    }

    let mut consecutive = 1;
    for i in (1..chars.len()).rev() {
        if chars[i] == chars[i - 1] && chars[i].is_ascii_alphanumeric() {
            consecutive += 1;
            if consecutive >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    false
}

/// 打印到控制台
pub fn print_address(addr: &VanityAddress, is_vanity: bool) {
    use colored::*;

    if is_vanity {
        println!(
            "{}",
            "╔════════════════════════════════════════════════════════════╗".bright_yellow()
        );
        println!(
            "{} {}",
            "║ 发现靓号! | Found Vanity Address! |".bright_yellow(),
            "".bright_yellow()
        );
        println!(
            "{}",
            "╠════════════════════════════════════════════════════════════╣".bright_yellow()
        );
        println!("{} {}", "链 | Chain:".bright_green(), addr.chain.label());
        println!(
            "{} {}",
            "地址 | Address:".bright_green(),
            addr.address.bright_cyan()
        );
        println!(
            "{} {}",
            "私钥 | Private Key:".bright_red(),
            addr.private_key.bright_white()
        );
        println!("{} {}", "公钥 | Public Key:".bright_blue(), addr.public_key);
        println!(
            "{} {}",
            "助记词 | Mnemonic:".bright_magenta(),
            addr.mnemonic
        );
        println!(
            "{}",
            "╚════════════════════════════════════════════════════════════╝".bright_yellow()
        );
    } else {
        println!("{} {}", "地址 | Address:".bright_green(), addr.address);
    }
}

/// 写入文件
pub fn save_address_to_file(
    filename: &str,
    addr: &VanityAddress,
    is_vanity: bool,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let vanity_mark = if is_vanity { "[VANITY]" } else { "[NORMAL]" };

    writeln!(
        file,
        "═══════════════════════════════════════════════════════════"
    )?;
    writeln!(
        file,
        "{} {} | Chain: {}",
        vanity_mark,
        timestamp,
        addr.chain.label()
    )?;
    writeln!(file, "Address: {}", addr.address)?;
    writeln!(file, "Private Key: {}", addr.private_key)?;
    writeln!(file, "Public Key: {}", addr.public_key)?;
    writeln!(file, "Mnemonic: {}", addr.mnemonic)?;
    writeln!(
        file,
        "═══════════════════════════════════════════════════════════"
    )?;
    writeln!(file)?;

    Ok(())
}

/// 获取十六进制字符串
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tron_address_generation() {
        let addr = generate_tron_address();
        assert!(addr.address.starts_with('T'));
        assert_eq!(addr.private_key.len(), 64);
    }

    #[test]
    fn test_evm_address_generation() {
        let addr = generate_evm_address();
        assert!(addr.address.starts_with("0x"));
        assert_eq!(addr.address.len(), 42);
    }

    #[test]
    fn test_sol_address_generation() {
        let addr = generate_sol_address();
        assert!(addr.address.len() >= 32); // Solana 地址 Base58 长度不固定但>=32
    }

    #[test]
    fn test_vanity_detection() {
        assert!(is_vanity_address("T1234567890aaa", &[]));
        assert!(is_vanity_address("0xabc1111", &[]));
        assert!(!is_vanity_address("Taaa1234567890", &[]));
    }

    #[test]
    fn test_custom_pattern_detection() {
        let patterns = &["lucky"];
        assert!(is_vanity_address("TAbCDEFlucky", patterns));
        assert!(!is_vanity_address("luckyABC", patterns));
    }

    #[test]
    fn test_mnemonic_generation() {
        let addr = generate_tron_address();
        let words: Vec<&str> = addr.mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), 12);
    }
}

use std::{fs::OpenOptions, io::Write};

use chrono::Local;
use ed25519_dalek::{PublicKey, SecretKey};
use ed25519_dalek_bip32::{DerivationPath as Ed25519DerivationPath, ExtendedSecretKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand::RngCore;
use rust_embed::RustEmbed;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use bip39::Mnemonic;

pub mod monitor;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Assets;

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

/// 多链地址结构：一个助记词对应的三种链地址
#[derive(Clone, Debug)]
pub struct MultiChainAddress {
    pub mnemonic: String,
    pub tron: VanityAddress,
    pub evm: VanityAddress,
    pub sol: VanityAddress,
}

/// 从助记词派生种子（BIP39）
fn mnemonic_to_seed(mnemonic: &str, password: &str) -> [u8; 64] {
    let parsed = Mnemonic::parse(mnemonic).expect("invalid mnemonic");
    parsed.to_seed(password)
}

/// 从种子派生 secp256k1 私钥（BIP44 标准派生）
fn derive_private_key_from_seed_bip44(seed: &[u8; 64], coin_type: u32) -> [u8; 32] {
    use bip32::{DerivationPath, XPrv};
    use core::str::FromStr;

    let path = DerivationPath::from_str(&format!("m/44'/{}'/0'/0/0", coin_type))
        .expect("valid path");

    let child = XPrv::derive_from_path(seed, &path).expect("derive child");
    child.private_key().to_bytes().into()
}

/// 从种子派生 TRON 私钥（m/44'/195'/0'/0/0）
fn derive_tron_private_key(seed: &[u8; 64]) -> [u8; 32] {
    derive_private_key_from_seed_bip44(seed, 195) // TRON coin type = 195
}

/// 从种子派生 EVM 私钥（m/44'/60'/0'/0/0）
fn derive_evm_private_key(seed: &[u8; 64]) -> [u8; 32] {
    derive_private_key_from_seed_bip44(seed, 60) // Ethereum coin type = 60
}

/// 从种子派生 Solana 私钥（Phantom 等钱包默认使用 m/44'/501'/0'）
fn derive_sol_private_key(seed: &[u8; 64]) -> [u8; 32] {
    use core::str::FromStr;

    let path = Ed25519DerivationPath::from_str("m/44'/501'/0'")
        .expect("valid solana path");
    let extended = ExtendedSecretKey::from_seed(seed).expect("valid solana seed");
    let derived = extended.derive(&path).expect("derive sol key");
    derived.secret_key.to_bytes()
}

/// 生成随机助记词
pub fn generate_mnemonic() -> String {
    let mut entropy = [0u8; 16]; // 128 bits = 12 words
    rand::thread_rng().fill_bytes(&mut entropy);
    match Mnemonic::from_entropy(&entropy) {
        Ok(m) => m.to_string(),
        Err(_) => "unable to generate mnemonic".to_string(),
    }
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
    // 1. Keccak256 公钥（跳过 0x04 前缀），取后 20 字节
    let mut keccak = Keccak::v256();
    keccak.update(&public_key[1..]);
    let mut out = [0u8; 32];
    keccak.finalize(&mut out);
    let account = &out[12..];

    // 2. 添加 Tron 主网前缀 0x41
    let mut versioned = vec![0x41];
    versioned.extend_from_slice(account);

    // 3. 双 SHA256 取前 4 字节做校验和
    let mut hasher = Sha256::new();
    hasher.update(&versioned);
    let sha256_1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&sha256_1);
    let sha256_2 = hasher.finalize();

    let checksum = &sha256_2[0..4];

    // 4. 拼接并 Base58 编码
    let mut address_bytes = versioned;
    address_bytes.extend_from_slice(checksum);

    bs58::encode(&address_bytes).into_string()
}

/// 将 20 字节地址编码为 EIP-55 校验格式
fn evm_checksum_address(address_bytes: &[u8]) -> String {
    let hex_lower = hex::encode(address_bytes);
    let mut keccak = Keccak::v256();
    keccak.update(hex_lower.as_bytes());
    let mut hash = [0u8; 32];
    keccak.finalize(&mut hash);

    let mut result = String::with_capacity(42);
    result.push_str("0x");

    for (i, ch) in hex_lower.chars().enumerate() {
        let hash_byte = hash[i / 2];
        let hash_nibble = if i % 2 == 0 {
            (hash_byte >> 4) & 0x0f
        } else {
            hash_byte & 0x0f
        };

        if ch.is_ascii_alphabetic() && hash_nibble >= 8 {
            result.push(ch.to_ascii_uppercase());
        } else {
            result.push(ch.to_ascii_lowercase());
        }
    }

    result
}

/// 生成 TRON 地址
pub fn generate_tron_address() -> VanityAddress {
    // 1. 先生成助记词
    let mnemonic = generate_mnemonic();
    
    // 2. 从助记词派生种子
    let seed = mnemonic_to_seed(&mnemonic, "");
    
    // 3. 从种子派生 TRON 私钥 (BIP44 m/44'/195'/0'/0/0)
    let private_key = derive_tron_private_key(&seed);
    
    // 4. 从私钥生成公钥和地址
    let public_key = private_key_to_public_key(&private_key);
    let address = public_key_to_tron_address(&public_key);

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
    // 1. 先生成助记词
    let mnemonic = generate_mnemonic();
    
    // 2. 从助记词派生种子
    let seed = mnemonic_to_seed(&mnemonic, "");
    
    // 3. 从种子派生 EVM 私钥 (BIP44 m/44'/60'/0'/0/0)
    let private_key = derive_evm_private_key(&seed);
    
    // 4. 从私钥生成公钥和地址
    let public_key = private_key_to_public_key(&private_key);

    // keccak256 公钥（去掉 0x04 前缀）后取后 20 字节
    let mut keccak = Keccak::v256();
    keccak.update(&public_key[1..]);
    let mut out = [0u8; 32];
    keccak.finalize(&mut out);
    let address_bytes = &out[12..];
    let address = evm_checksum_address(address_bytes);

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
    // 1. 先生成助记词
    let mnemonic = generate_mnemonic();
    
    // 2. 从助记词派生种子
    let seed_64 = mnemonic_to_seed(&mnemonic, "");
    
    // 3. 从种子派生 Solana 私钥 (BIP44 m/44'/501'/0'/0/0)
    let seed = derive_sol_private_key(&seed_64);

    let secret = SecretKey::from_bytes(&seed).expect("valid seed");
    let public: PublicKey = (&secret).into();

    let address = bs58::encode(public.as_bytes()).into_string();

    VanityAddress {
        chain: ChainType::Sol,
        address,
        public_key: hex::encode(public.as_bytes()),
        private_key: hex::encode(seed),
        mnemonic,
    }
}

/// 从单个助记词生成所有三种链的地址
pub fn generate_from_mnemonic_all(mnemonic: &str) -> MultiChainAddress {
    let seed = mnemonic_to_seed(mnemonic, "");
    
    // TRON
    let tron_priv = derive_tron_private_key(&seed);
    let tron_pub = private_key_to_public_key(&tron_priv);
    let tron_addr = public_key_to_tron_address(&tron_pub);
    let tron = VanityAddress {
        chain: ChainType::Tron,
        address: tron_addr,
        public_key: hex::encode(&tron_pub),
        private_key: hex::encode(&tron_priv),
        mnemonic: mnemonic.to_string(),
    };
    
    // EVM
    let evm_priv = derive_evm_private_key(&seed);
    let evm_pub = private_key_to_public_key(&evm_priv);
    let mut keccak = Keccak::v256();
    keccak.update(&evm_pub[1..]);
    let mut out = [0u8; 32];
    keccak.finalize(&mut out);
    let evm_addr = evm_checksum_address(&out[12..]);
    let evm = VanityAddress {
        chain: ChainType::Evm,
        address: evm_addr,
        public_key: hex::encode(&evm_pub),
        private_key: hex::encode(&evm_priv),
        mnemonic: mnemonic.to_string(),
    };
    
    // Solana
    let sol_priv = derive_sol_private_key(&seed);
    let sol_secret = SecretKey::from_bytes(&sol_priv).expect("valid seed");
    let sol_pub: PublicKey = (&sol_secret).into();
    let sol_addr = bs58::encode(sol_pub.as_bytes()).into_string();
    let sol = VanityAddress {
        chain: ChainType::Sol,
        address: sol_addr,
        public_key: hex::encode(sol_pub.as_bytes()),
        private_key: hex::encode(&sol_priv),
        mnemonic: mnemonic.to_string(),
    };
    
    MultiChainAddress {
        mnemonic: mnemonic.to_string(),
        tron,
        evm,
        sol,
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

/// 打印多链地址到控制台（命中任意链时，列出三条链）
pub fn print_multi_address(multi: &MultiChainAddress, hit_chain: ChainType) {
    use colored::*;

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
    println!(
        "{} {}",
        "命中链 | Hit Chain:".bright_green(),
        hit_chain.label()
    );

    println!(
        "{} {}",
        "TRON 地址:".bright_green(),
        multi.tron.address.bright_cyan()
    );
    println!(
        "{} {}",
        "EVM 地址:".bright_green(),
        multi.evm.address.bright_cyan()
    );
    println!(
        "{} {}",
        "SOL 地址:".bright_green(),
        multi.sol.address.bright_cyan()
    );

    println!(
        "{} {}",
        "TRON 私钥:".bright_red(),
        multi.tron.private_key.bright_red()
    );
    println!(
        "{} {}",
        "EVM 私钥:".bright_red(),
        multi.evm.private_key.bright_red()
    );
    println!(
        "{} {}",
        "SOL 私钥:".bright_red(),
        multi.sol.private_key.bright_red()
    );

    println!(
        "{} {}",
        "助记词 | Mnemonic:".bright_magenta(),
        multi.mnemonic.bright_white()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_yellow()
    );
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

/// 写入多链地址到文件（命中任意链时一次性记录三条链）
pub fn save_multi_address_to_file(
    filename: &str,
    multi: &MultiChainAddress,
    hit_chain: ChainType,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    writeln!(
        file,
        "═══════════════════════════════════════════════════════════"
    )?;
    writeln!(
        file,
        "[VANITY] {} | Hit Chain: {}",
        timestamp,
        hit_chain.label()
    )?;
    writeln!(file, "TRON Address: {}", multi.tron.address)?;
    writeln!(file, "EVM Address: {}", multi.evm.address)?;
    writeln!(file, "SOL Address: {}", multi.sol.address)?;
    writeln!(file, "TRON Private Key: {}", multi.tron.private_key)?;
    writeln!(file, "EVM Private Key: {}", multi.evm.private_key)?;
    writeln!(file, "SOL Private Key: {}", multi.sol.private_key)?;
    writeln!(file, "Mnemonic: {}", multi.mnemonic)?;
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
    fn test_known_mnemonic_addresses() {
        let mnemonic = "scissors inch embody vapor garment panther cinnamon theme first coast panda brand";
        let multi = generate_from_mnemonic_all(mnemonic);

        assert_eq!(multi.evm.address, "0x1D2F71D84cB6fE09B06F86F5bf18e498526a7Fb1");
        assert_eq!(multi.tron.address, "TGu44ECEQD9YnG7gkV9paBpbKgKwQnCNN5");
        assert_eq!(multi.sol.address, "3Xa9gJdvWpuSnUyAs34EhFVzA1Lk8Mjs8LYNRnWVWonS");
    }

    #[test]
    fn test_mnemonic_generation() {
        let addr = generate_tron_address();
        let words: Vec<&str> = addr.mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), 12);
    }
}

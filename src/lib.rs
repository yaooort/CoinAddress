use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub mod monitor;

/// TRON地址相关信息
#[derive(Clone, Debug)]
pub struct TronAddress {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub mnemonic: String,
}

/// 生成随机私钥
pub fn generate_private_key() -> [u8; 32] {
    use rand::RngCore;
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

/// 从私钥生成公钥
pub fn private_key_to_public_key(private_key: &[u8; 32]) -> Vec<u8> {
    use k256::SecretKey;
    let secret = SecretKey::from_slice(private_key).expect("valid key");
    let public = secret.public_key();
    public.to_encoded_point(false).as_bytes().to_vec()
}

/// 从公钥生成TRON地址
pub fn public_key_to_address(public_key: &[u8]) -> String {
    // 1. SHA256哈希公钥
    let mut hasher = Sha256::new();
    hasher.update(&public_key[1..]); // 跳过前缀字节
    let sha256_hash = hasher.finalize();

    // 2. RIPEMD160哈希
    let mut hasher = Ripemd160::new();
    hasher.update(&sha256_hash);
    let ripemd160_hash = hasher.finalize();

    // 3. 添加版本字节（0x41 for TRON mainnet）
    let mut versioned = vec![0x41];
    versioned.extend_from_slice(&ripemd160_hash);

    // 4. 计算校验和（SHA256两次）
    let mut hasher = Sha256::new();
    hasher.update(&versioned);
    let sha256_1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&sha256_1);
    let sha256_2 = hasher.finalize();

    // 5. 取前4字节作为校验和
    let checksum = &sha256_2[0..4];

    // 6. 添加校验和
    let mut address_bytes = versioned;
    address_bytes.extend_from_slice(checksum);

    // 7. Base58编码
    bs58::encode(&address_bytes).into_string()
}

/// 生成TRON地址及相关信息
pub fn generate_tron_address() -> TronAddress {
    // 生成私钥
    let private_key = generate_private_key();
    let private_key_hex = hex::encode(&private_key);

    // 生成公钥
    let public_key = private_key_to_public_key(&private_key);
    let public_key_hex = hex::encode(&public_key);

    // 生成地址
    let address = public_key_to_address(&public_key);

    // 生成BIP39助记词
    let entropy = private_key.to_vec();
    let mnemonic = generate_mnemonic(&entropy);

    TronAddress {
        address,
        public_key: public_key_hex,
        private_key: private_key_hex,
        mnemonic,
    }
}

/// 生成BIP39助记词
fn generate_mnemonic(entropy: &[u8]) -> String {
    use bip39::Mnemonic;
    // 使用前16字节生成12个单词的助记词
    match Mnemonic::from_entropy(&entropy[0..16]) {
        Ok(m) => m.to_string(),
        Err(_) => "unable to generate mnemonic".to_string(),
    }
}

/// 检查是否为靓号（美号）
/// 检查标准：末尾匹配指定模式或连续相同字符
pub fn is_vanity_address(address: &str, patterns: &[&str]) -> bool {
    let address_lower = address.to_lowercase();
    
    // 如果有自定义模式，检查末尾是否匹配
    if !patterns.is_empty() {
        for pattern in patterns {
            let pattern_lower = pattern.to_lowercase();
            if address_lower.ends_with(&pattern_lower) {
                return true;
            }
        }
        return false;
    }
    
    // 没有自定义模式时，检查末尾是否有连续相同字符（3个或以上）
    let chars: Vec<char> = address_lower.chars().collect();
    if chars.len() < 3 {
        return false;
    }
    
    // 检查末尾的连续相同字符
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

/// 检查地址中是否包含特定前缀
pub fn has_prefix(address: &str, prefix: &str) -> bool {
    address.to_lowercase().contains(&prefix.to_lowercase())
}

/// 打印地址到控制台
pub fn print_address(tron: &TronAddress, is_vanity: bool) {
    use colored::*;
    
    if is_vanity {
        println!("{}", "╔════════════════════════════════════════════════════════════╗".bright_yellow());
        println!("{} {}", "║ 发现靓号! | Found Vanity Address! |".bright_yellow(), "".bright_yellow());
        println!("{}", "╠════════════════════════════════════════════════════════════╣".bright_yellow());
        println!("{} {}", "地址 | Address:".bright_green(), tron.address.bright_cyan());
        println!("{} {}", "私钥 | Private Key:".bright_red(), tron.private_key.bright_white());
        println!("{} {}", "公钥 | Public Key:".bright_blue(), tron.public_key);
        println!("{} {}", "助记词 | Mnemonic:".bright_magenta(), tron.mnemonic);
        println!("{}", "╚════════════════════════════════════════════════════════════╝".bright_yellow());
    } else {
        println!("{} {}", "地址 | Address:".bright_green(), tron.address);
    }
}

/// 打印地址到文件
pub fn save_address_to_file(filename: &str, tron: &TronAddress, is_vanity: bool) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let vanity_mark = if is_vanity { "[VANITY]" } else { "[NORMAL]" };

    writeln!(file, "═══════════════════════════════════════════════════════════")?;
    writeln!(file, "{} {}", vanity_mark, timestamp)?;
    writeln!(file, "Address: {}", tron.address)?;
    writeln!(file, "Private Key: {}", tron.private_key)?;
    writeln!(file, "Public Key: {}", tron.public_key)?;
    writeln!(file, "Mnemonic: {}", tron.mnemonic)?;
    writeln!(file, "═══════════════════════════════════════════════════════════")?;
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
        assert!(addr.address.starts_with("T"));
        assert_eq!(addr.private_key.len(), 64); // 32字节 = 64个十六进制字符
    }

    #[test]
    fn test_vanity_detection() {
        // 末尾3个或以上连续相同字符才算靓号
        assert!(is_vanity_address("T1234567890aaa", &[]));
        assert!(is_vanity_address("Tabc1111", &[]));
        assert!(!is_vanity_address("Taaa1234567890", &[]));
        assert!(!is_vanity_address("T1234567890", &[]));
    }

    #[test]
    fn test_address_format() {
        let addr = generate_tron_address();
        assert!(addr.address.starts_with("T"));
        assert_eq!(addr.address.len(), 34);
    }

    #[test]
    fn test_private_key_format() {
        let addr = generate_tron_address();
        // 私钥应该是64个十六进制字符
        assert_eq!(addr.private_key.len(), 64);
        assert!(addr.private_key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_mnemonic_generation() {
        let addr = generate_tron_address();
        let words: Vec<&str> = addr.mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), 12);
    }

    #[test]
    fn test_custom_pattern_detection() {
        // 测试末尾匹配
        let patterns = &["lucky"];
        assert!(is_vanity_address("TAbCDEFlucky", patterns));
        assert!(is_vanity_address("Tlucky", patterns));
        assert!(!is_vanity_address("TluckyABC", patterns));
        assert!(!is_vanity_address("TPoorAddress", patterns));
    }

    #[test]
    fn test_vanity_default_pattern() {
        // 测试默认靓号检测（末尾连续相同字符3个或以上）
        assert!(is_vanity_address("TAbcd1111", &[]));
        assert!(is_vanity_address("Ttest6666", &[]));
        assert!(!is_vanity_address("TAbcd11", &[])); // 只有2个，不符合
        assert!(!is_vanity_address("T1111abcd", &[])); // 开头不算
    }
}

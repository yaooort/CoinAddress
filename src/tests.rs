#[cfg(test)]
mod tests {
    use tron_vanity::*;

    #[test]
    fn test_address_generation() {
        let addr = generate_tron_address();
        
        // 地址应该以 T 开头
        assert!(addr.address.starts_with("T"), "Address should start with T");
        
        // 地址长度应该是34
        assert_eq!(addr.address.len(), 34, "Address should be 34 characters");
        
        // 私钥应该是64个十六进制字符（32字节）
        assert_eq!(addr.private_key.len(), 64, "Private key should be 64 hex chars");
        
        // 公钥应该以 04 开头（非压缩格式）
        assert!(addr.public_key.starts_with("04"), "Public key should start with 04");
        
        // 助记词应该包含12个单词
        let words: Vec<&str> = addr.mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), 12, "Mnemonic should have 12 words");
    }

    #[test]
    fn test_private_key_generation() {
        let key1 = generate_private_key();
        let key2 = generate_private_key();
        
        // 不同的调用应该生成不同的私钥
        assert_ne!(key1, key2, "Generated keys should be different");
        
        // 私钥长度应该是32字节
        assert_eq!(key1.len(), 32, "Private key should be 32 bytes");
    }

    #[test]
    fn test_vanity_detection_consecutive() {
        // 测试连续相同字符检测
        assert!(is_vanity_address("Taaa123456789", &[]), "Should detect 'aaa'");
        assert!(is_vanity_address("T1111", &[]), "Should detect '1111'");
        assert!(is_vanity_address("TBBBB", &[]), "Should detect 'BBBB'");
    }

    #[test]
    fn test_vanity_detection_sequential() {
        // 测试递增序列检测
        assert!(is_vanity_address("T123xyz", &[]), "Should detect '123'");
        assert!(is_vanity_address("TAbcxyz", &[]), "Should detect 'abc'");
    }

    #[test]
    fn test_vanity_detection_custom() {
        // 测试自定义模式检测
        let patterns = &["Lucky", "Rich", "Moon"];
        
        assert!(is_vanity_address("TLuckyDFKJL", patterns), "Should detect 'Lucky'");
        assert!(is_vanity_address("TRichDFKJL", patterns), "Should detect 'Rich'");
        assert!(is_vanity_address("TMoonDFKJL", patterns), "Should detect 'Moon'");
        
        assert!(!is_vanity_address("TPoorDFKJL", patterns), "Should not detect 'Poor'");
    }

    #[test]
    fn test_prefix_check() {
        assert!(has_prefix("TLuckyAddress", "Lucky"), "Should contain 'Lucky'");
        assert!(has_prefix("T1111Address", "1111"), "Should contain '1111'");
        assert!(!has_prefix("TAddress", "Lucky"), "Should not contain 'Lucky'");
    }

    #[test]
    fn test_hex_encoding() {
        let data = vec![0xAB, 0xCD, 0xEF];
        let hex = hex_encode(&data);
        assert_eq!(hex, "abcdef", "Hex encoding should match");
    }

    #[test]
    fn test_multiple_addresses() {
        // 生成多个地址，确保都有效
        for _ in 0..10 {
            let addr = generate_tron_address();
            assert!(addr.address.starts_with("T"), "All addresses should start with T");
            assert_eq!(addr.address.len(), 34, "All addresses should be 34 chars");
        }
    }

    #[test]
    fn test_public_key_to_address() {
        // 测试公钥到地址的转换
        let private_key = generate_private_key();
        let public_key = private_key_to_public_key(&private_key);
        let address = public_key_to_address(&public_key);
        
        assert!(address.starts_with("T"), "Generated address should start with T");
        assert_eq!(address.len(), 34, "Generated address should be 34 characters");
    }

    #[test]
    fn test_vanity_address_properties() {
        // 找一个靓号并验证其属性
        let mut found = false;
        for _ in 0..1000 {
            let addr = generate_tron_address();
            if is_vanity_address(&addr.address, &[]) {
                found = true;
                
                // 验证所有字段都存在且有效
                assert!(!addr.address.is_empty());
                assert!(!addr.private_key.is_empty());
                assert!(!addr.public_key.is_empty());
                assert!(!addr.mnemonic.is_empty());
                break;
            }
        }
        
        // 注意：可能找不到靓号，这不是测试失败
        if !found {
            println!("No vanity address found in 1000 attempts (this is expected)");
        }
    }
}

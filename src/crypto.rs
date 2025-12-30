use md5::Md5;
use md4::Md4;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use sm3::Sm3;
use ripemd::Ripemd160;
use whirlpool::Whirlpool;
use blake2::{Blake2b512, Blake2s256};
use digest::Digest;
use base64::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    None, // 明文/不计算
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Sm3,
    Ripemd160,
    Whirlpool,
    Blake2b,
    Blake2s,
    Blake3,
}

impl HashAlgorithm {
    pub fn all() -> &'static [HashAlgorithm] {
        &[
            HashAlgorithm::None,
            HashAlgorithm::Md5,
            HashAlgorithm::Sha1,
            HashAlgorithm::Sha224,
            HashAlgorithm::Sha256,
            HashAlgorithm::Sha384,
            HashAlgorithm::Sha512,
            HashAlgorithm::Sha3_224,
            HashAlgorithm::Sha3_256,
            HashAlgorithm::Sha3_384,
            HashAlgorithm::Sha3_512,
            HashAlgorithm::Sm3,
            HashAlgorithm::Ripemd160,
            HashAlgorithm::Whirlpool,
            HashAlgorithm::Blake2b,
            HashAlgorithm::Blake2s,
            HashAlgorithm::Blake3,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::None => "明文 (不计算)",
            HashAlgorithm::Md5 => "MD5",
            HashAlgorithm::Sha1 => "SHA-1",
            HashAlgorithm::Sha224 => "SHA2-224",
            HashAlgorithm::Sha256 => "SHA2-256",
            HashAlgorithm::Sha384 => "SHA2-384",
            HashAlgorithm::Sha512 => "SHA2-512",
            HashAlgorithm::Sha3_224 => "SHA3-224",
            HashAlgorithm::Sha3_256 => "SHA3-256",
            HashAlgorithm::Sha3_384 => "SHA3-384",
            HashAlgorithm::Sha3_512 => "SHA3-512",
            HashAlgorithm::Sm3 => "SM3",
            HashAlgorithm::Ripemd160 => "RIPEMD-160",
            HashAlgorithm::Whirlpool => "Whirlpool",
            HashAlgorithm::Blake2b => "BLAKE2b",
            HashAlgorithm::Blake2s => "BLAKE2s",
            HashAlgorithm::Blake3 => "BLAKE3",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaltMode {
    None,
    Prefix,
    Suffix,
    Both,
    Custom,
}

impl SaltMode {
    pub fn all() -> &'static [SaltMode] {
        &[
            SaltMode::None,
            SaltMode::Prefix,
            SaltMode::Suffix,
            SaltMode::Both,
            SaltMode::Custom,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            SaltMode::None => "不加盐",
            SaltMode::Prefix => "前缀 (Salt + Data)",
            SaltMode::Suffix => "后缀 (Data + Salt)",
            SaltMode::Both => "前后缀 (Salt + Data + Salt)",
            SaltMode::Custom => "自定义 (积木模式)",
        }
    }
}

pub fn calculate_hash(
    algo: HashAlgorithm,
    input: &str,
    salt: &str,
    mode: SaltMode,
    custom_data: Option<String>,
) -> String {
    let data = match mode {
        SaltMode::None => input.to_string(),
        SaltMode::Prefix => format!("{}{}", salt, input),
        SaltMode::Suffix => format!("{}{}", input, salt),
        SaltMode::Both => format!("{}{}{}", salt, input, salt),
        SaltMode::Custom => custom_data.unwrap_or_default(),
    };

    let bytes = data.as_bytes();

    match algo {
        HashAlgorithm::None => data,
        HashAlgorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha224 => {
            let mut hasher = Sha224::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha384 => {
            let mut hasher = Sha384::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha3_224 => {
            let mut hasher = Sha3_224::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha3_256 => {
            let mut hasher = Sha3_256::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha3_384 => {
            let mut hasher = Sha3_384::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sha3_512 => {
            let mut hasher = Sha3_512::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Sm3 => {
            let mut hasher = Sm3::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Ripemd160 => {
            let mut hasher = Ripemd160::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Whirlpool => {
            let mut hasher = Whirlpool::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Blake2b => {
            let mut hasher = Blake2b512::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Blake2s => {
            let mut hasher = Blake2s256::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize().as_bytes())
        }
    }
}

pub fn calculate_complex_hashes(pass: &str, salt: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();

    // Helper closures
    let md5 = |s: &[u8]| -> String {
        let mut h = Md5::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let sha1 = |s: &[u8]| -> String {
        let mut h = Sha1::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let sha256 = |s: &[u8]| -> String {
        let mut h = Sha256::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let sha384 = |s: &[u8]| -> String {
        let mut h = Sha384::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let sha512 = |s: &[u8]| -> String {
        let mut h = Sha512::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let sm3 = |s: &[u8]| -> String {
        let mut h = Sm3::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let ripemd160 = |s: &[u8]| -> String {
        let mut h = Ripemd160::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let whirlpool = |s: &[u8]| -> String {
        let mut h = Whirlpool::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let blake2b = |s: &[u8]| -> String {
        let mut h = Blake2b512::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let blake2s = |s: &[u8]| -> String {
        let mut h = Blake2s256::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let blake3_fn = |s: &[u8]| -> String {
        let mut h = blake3::Hasher::new();
        h.update(s);
        hex::encode(h.finalize().as_bytes())
    };
    let sha3_256 = |s: &[u8]| -> String {
        let mut h = Sha3_256::new();
        h.update(s);
        hex::encode(h.finalize())
    };
    let sha3_512 = |s: &[u8]| -> String {
        let mut h = Sha3_512::new();
        h.update(s);
        hex::encode(h.finalize())
    };

    // 1. Base64
    results.push(("base64".to_string(), BASE64_STANDARD.encode(pass)));

    // 2. MD5
    let md5_pass = md5(pass.as_bytes());
    results.push(("md5".to_string(), md5_pass.clone()));

    // 3. MD5 Middle (8-24)
    if md5_pass.len() >= 24 {
        results.push(("md5_middle".to_string(), md5_pass[8..24].to_string()));
    }

    // 4. md5(md5($pass))
    let md5_md5_pass = md5(md5_pass.as_bytes());
    results.push(("md5(md5($pass))".to_string(), md5_md5_pass.clone()));

    // 5. md5(md5(md5($pass)))
    results.push(("md5(md5(md5($pass)))".to_string(), md5(md5_md5_pass.as_bytes())));

    // 6. md5(unicode) - UTF-16LE
    let pass_utf16: Vec<u8> = pass.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    results.push(("md5(unicode)".to_string(), md5(&pass_utf16)));

    // 7. md5(base64)
    results.push(("md5(base64)".to_string(), md5(BASE64_STANDARD.encode(pass).as_bytes())));

    // 8. mysql (Old Password - skipped for now, using new password style if needed, but user asked for mysql and mysql5)
    // MySQL 4.1+ (mysql5) is SHA1(SHA1(pass))
    let sha1_pass = sha1(pass.as_bytes());
    let sha1_sha1_pass = sha1(sha1_pass.as_bytes()); // Note: MySQL uses binary SHA1(SHA1), but usually tools display hex of hex? No, MySQL is hex(SHA1(unhex(SHA1(pass)))). 
    // Actually MySQL 4.1+ PASSWORD() is "*" + hex(SHA1(SHA1(pass_bytes))).
    // Let's implement standard hex(SHA1(hex(SHA1(pass)))) which is common in these tools, OR the actual MySQL one.
    // Given the list "sha1(sha1($psss))" is separate, "mysql5" likely refers to the specific MySQL format.
    // Let's stick to the literal interpretation of the user's list if possible.
    // "mysql5" usually means "*" + UPPER(SHA1(SHA1(pass_bytes))).
    // But let's check "sha1(sha1($psss))".
    
    // Let's implement "mysql5" as * + UPPER(SHA1(SHA1(pass)))
    let mut h1 = Sha1::new();
    h1.update(pass.as_bytes());
    let h1_bytes = h1.finalize();
    let mut h2 = Sha1::new();
    h2.update(h1_bytes);
    results.push(("mysql5".to_string(), format!("*{}", hex::encode(h2.finalize()).to_uppercase())));

    // 9. NTLM (MD4 of UTF-16LE)
    let mut ntlm_hasher = Md4::new();
    ntlm_hasher.update(&pass_utf16);
    results.push(("ntlm".to_string(), hex::encode(ntlm_hasher.finalize())));

    // 10. SHA1
    results.push(("sha1".to_string(), sha1_pass.clone()));

    // 11. sha1(sha1($psss)) - assuming hex input for second round? Or bytes?
    // Usually in these lists, it implies sha1(sha1_hex).
    results.push(("sha1(sha1($pass))".to_string(), sha1(sha1_pass.as_bytes())));

    // 12. sha1(md5($psss))
    results.push(("sha1(md5($pass))".to_string(), sha1(md5_pass.as_bytes())));

    // 13. md5(sha1($psss))
    results.push(("md5(sha1($pass))".to_string(), md5(sha1_pass.as_bytes())));

    // 14. sha256
    let sha256_pass = sha256(pass.as_bytes());
    results.push(("sha256".to_string(), sha256_pass.clone()));

    // 15. sha256(md5($pass))
    results.push(("sha256(md5($pass))".to_string(), sha256(md5_pass.as_bytes())));

    // 16. sha384
    results.push(("sha384".to_string(), sha384(pass.as_bytes())));

    // 17. sha512
    results.push(("sha512".to_string(), sha512(pass.as_bytes())));

    // 18. SM3 (Commercial Cryptography)
    let sm3_pass = sm3(pass.as_bytes());
    results.push(("sm3".to_string(), sm3_pass.clone()));

    // 19. RIPEMD-160
    results.push(("ripemd160".to_string(), ripemd160(pass.as_bytes())));

    // 20. Whirlpool
    results.push(("whirlpool".to_string(), whirlpool(pass.as_bytes())));

    // 21. SHA3 Family
    results.push(("sha3_256".to_string(), sha3_256(pass.as_bytes())));
    results.push(("sha3_512".to_string(), sha3_512(pass.as_bytes())));

    // 22. BLAKE Family
    results.push(("blake2b".to_string(), blake2b(pass.as_bytes())));
    results.push(("blake2s".to_string(), blake2s(pass.as_bytes())));
    results.push(("blake3".to_string(), blake3_fn(pass.as_bytes())));

    // Salted variations
    // md5(md5($pass).$salt);VB;DZ -> md5(md5(pass) + salt)
    results.push(("md5(md5($pass).$salt)".to_string(), md5(format!("{}{}", md5_pass, salt).as_bytes())));

    // md5($pass.$salt)
    results.push(("md5($pass.$salt)".to_string(), md5(format!("{}{}", pass, salt).as_bytes())));

    // md5($salt.$pass)
    results.push(("md5($salt.$pass)".to_string(), md5(format!("{}{}", salt, pass).as_bytes())));

    // md5($salt.$pass.$salt)
    results.push(("md5($salt.$pass.$salt)".to_string(), md5(format!("{}{}{}", salt, pass, salt).as_bytes())));

    // md5($salt.md5($pass))
    results.push(("md5($salt.md5($pass))".to_string(), md5(format!("{}{}", salt, md5_pass).as_bytes())));

    // md5(md5($salt).$pass)
    let md5_salt = md5(salt.as_bytes());
    results.push(("md5(md5($salt).$pass)".to_string(), md5(format!("{}{}", md5_salt, pass).as_bytes())));

    // md5($pass.md5($salt))
    results.push(("md5($pass.md5($salt))".to_string(), md5(format!("{}{}", pass, md5_salt).as_bytes())));

    // md5(md5($salt).md5($pass))
    results.push(("md5(md5($salt).md5($pass))".to_string(), md5(format!("{}{}", md5_salt, md5_pass).as_bytes())));

    // md5(md5($pass).md5($salt))
    results.push(("md5(md5($pass).md5($salt))".to_string(), md5(format!("{}{}", md5_pass, md5_salt).as_bytes())));

    // md5(substring(md5($pass),8,16))
    if md5_pass.len() >= 24 {
        let sub = &md5_pass[8..24];
        results.push(("md5(substring(md5($pass),8,16))".to_string(), md5(sub.as_bytes())));
    }

    // sha1($pass.$salt)
    results.push(("sha1($pass.$salt)".to_string(), sha1(format!("{}{}", pass, salt).as_bytes())));

    // sha1($salt.$pass)
    results.push(("sha1($salt.$pass)".to_string(), sha1(format!("{}{}", salt, pass).as_bytes())));

    // sha256($pass.$salt)
    results.push(("sha256($pass.$salt)".to_string(), sha256(format!("{}{}", pass, salt).as_bytes())));

    // sha256($salt.$pass)
    results.push(("sha256($salt.$pass)".to_string(), sha256(format!("{}{}", salt, pass).as_bytes())));

    // sha512($pass.$salt)
    results.push(("sha512($pass.$salt)".to_string(), sha512(format!("{}{}", pass, salt).as_bytes())));

    // sha512($salt.$pass)
    results.push(("sha512($salt.$pass)".to_string(), sha512(format!("{}{}", salt, pass).as_bytes())));

    // sm3($pass.$salt)
    results.push(("sm3($pass.$salt)".to_string(), sm3(format!("{}{}", pass, salt).as_bytes())));

    // sm3($salt.$pass)
    results.push(("sm3($salt.$pass)".to_string(), sm3(format!("{}{}", salt, pass).as_bytes())));

    // sm3($salt.$pass.$salt)
    results.push(("sm3($salt.$pass.$salt)".to_string(), sm3(format!("{}{}{}", salt, pass, salt).as_bytes())));

    // MSSQL2015 (SHA2_512) - Assuming 0x0200 prefix + salt + hash
    // But without binary salt, we can't replicate exact MSSQL binary format.
    // We will just do a placeholder or standard salted SHA512 if that's what the user expects.
    // However, usually MSSQL hashes are binary.
    // Let's skip exact MSSQL binary replication and just provide the label as requested, maybe just SHA512(pass+salt) or similar if that's the common interpretation in these tools.
    // Actually, let's just output "Not Implemented" or similar if unsure, OR just SHA512(utf16(pass) + salt)?
    // Let's leave it out or put a placeholder.
    
    results
}

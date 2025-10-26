use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm,
};
use aes_gcm::aead::rand_core::RngCore;
use anyhow::{anyhow, Result};

pub struct Aes256GcmUtil {
    key: [u8; 32], // private key
    cipher: Aes256Gcm,
}

/**
 * anyhow="1.0.100"
 * rand="0.9.2"     # 仅用来打印 key/nonce 用 hex
 * aes-gcm="0.10.3" # AES-256-GCM         
 * hex="0.4.3"
 */
impl Aes256GcmUtil {
    pub fn new() -> Self {
        let mut key_bytes = [0u8; 32]; // private key
        // generate a random, cryptographically strong key
        OsRng.fill_bytes(&mut key_bytes);
        let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();
        Self { key: key_bytes, cipher }
    }

    pub fn export_hex_key(&self) -> String {
        hex::encode(self.key)
    }

    pub fn from_hex_key(key_hex: &str) -> Result<Self> {
        let key_bytes = hex::decode(key_hex)?;
        let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();
        Ok(Self {
            key: key_bytes.try_into().unwrap(),
            cipher,
        })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);

        let ciphertext = self
            .cipher
            .encrypt(&nonce.into(), plaintext)
            .map_err(|e| anyhow!("encryption failed: {:?}", e))?; 

        Ok((ciphertext, nonce.to_vec()))
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let plaintext = self
            .cipher
            .decrypt(nonce.into(), ciphertext)
            .map_err(|e| anyhow!("decryption failed: {:?}", e))?; 

        Ok(plaintext)
    }
}

pub fn test_Aes256GcmUtil() -> Result<()> {
    // Step 1. Create a new AES-256-GCM utility (random key)
    let util = Aes256GcmUtil::new();

    // Step 2. Encrypt a message
    let msg:&[u8] = "AES-256-GCM example".as_bytes(); // return array of pointers
    let (ct, nonce) = util.encrypt(msg)?;
    let key_hex = util.export_hex_key(); // export the key
    println!("cipher: {}", hex::encode(&ct));
    println!("nonce : {}", hex::encode(&nonce));
    println!("key   : {}", key_hex);

    // Step 3. Simulate using the same key later (like decrypting on another device)
    let util2 = Aes256GcmUtil::from_hex_key(&key_hex)?;
    let pt = util2.decrypt(&ct, &nonce)?;

    println!("plain  : {}", String::from_utf8(pt)?);

    Ok(())
}
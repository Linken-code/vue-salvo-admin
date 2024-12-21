use sha2::{Sha256, Digest};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let hashed = hash_password(password);
    hashed == hash
} 
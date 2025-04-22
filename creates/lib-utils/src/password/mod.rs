use argon2::Config;
use rand::{rng, Rng};

pub fn password_salt_hash(password: &str) -> (String, String) {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rng();
    let random_string: String = (0..CHARSET.len())
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let password = password.as_bytes();
    let salt = random_string.as_bytes();
    let hash = argon2::hash_encoded(password, salt, &Config::default()).unwrap();
    (hash, random_string)
}

pub fn verify_password(password: &str, salt: &str, hash_password: &str) -> bool {
    let hash =
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()).unwrap();
    let matches = argon2::verify_encoded(&hash, hash_password.as_bytes()).unwrap();
    matches
}

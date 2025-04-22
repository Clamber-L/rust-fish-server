use argon2::Config;
use rand::Rng;

pub fn password_salt_hash(password: &str) -> (String, String) {
    let rand = rand::rng().random();

    let password = password.as_bytes();
    let salt = b"randomsalt";
    let hash = argon2::hash_encoded(password, salt, &Config::default()).unwrap();
    let matches = argon2::verify_encoded(&hash, password).unwrap();
}

fn verify_password(password_hash: &str, salt: SaltString, password: &str) -> bool {
    let argon2 = Argon2::default();
    false
}

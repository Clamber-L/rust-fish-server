use lib_utils::password::*;

fn main() {
    let hash = password_salt_hash("password");
    println!("{:?}", hash);

    let matchs = verify_password("password", hash.1.as_str(), hash.0.as_str());
    println!("{:?}", matchs);
}

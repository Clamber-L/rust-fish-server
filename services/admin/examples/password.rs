use lib_utils::password::{password_salt_hash, verify_password};

fn main() {
    let password = password_salt_hash("admin");
    println!("{}", password);

    let matches = verify_password("admin", &password);
    println!("{}", matches);
}

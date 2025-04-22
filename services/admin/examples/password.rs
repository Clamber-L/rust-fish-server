use lib_utils::password::*;

fn main() {
    let hash = password_salt_hash("password");
    println!("{:?}", hash)
}

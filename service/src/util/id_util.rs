use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
pub fn generate_id() -> String {
    nanoid::nanoid!()
}

pub fn generate_password(pwd_len: i32) -> String {
    let mut rng = thread_rng();
    let password = (0..pwd_len)
        .map(|_| rng.sample(Alphanumeric))
        .collect::<Vec<u8>>();

    String::from_utf8(password).unwrap()
}
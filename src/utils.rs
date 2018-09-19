use rand::{thread_rng, Rng};

const HASH_LENGTH: u8 = 5;
const HASH_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn random_hash() -> Option<String> {
    let mut rng = thread_rng();
    let hash: Option<String> = (0..HASH_LENGTH)
        .map(|_| Some(*rng.choose(HASH_CHARSET)? as char))
        .collect();

    hash
}

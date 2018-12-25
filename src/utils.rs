use rand::seq::SliceRandom;
use rand::thread_rng;

const HASH_LENGTH: u8 = 5;
const HASH_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn random_hash() -> Option<String> {
    let mut rng = thread_rng();
    let hash: Option<String> = (0..HASH_LENGTH)
        .map(|_| Some(*HASH_CHARSET.choose(&mut rng)? as char))
        .collect();

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_hash() {
        assert_eq!(random_hash().unwrap().len(), HASH_LENGTH as usize);
    }
}

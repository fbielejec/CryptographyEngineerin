use std::collections::HashMap;

use sha3::{Digest, Sha3_512};

fn sha_512_n(message: &[u8], n: usize) -> Vec<u8> {
    let mut hasher = Sha3_512::new();
    hasher.update(message);
    hasher.finalize()[..n].into()
}

/// returns two messages that hash to the same sha512n value (if any)
fn exercise5_3(n: usize) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    (1usize..1_000_000).into_iter().find_map(|m| {
        let hash = sha_512_n(&m.to_le_bytes(), n);
        if let Some(previous) = map.insert(hash, m) {
            return Some((previous, m));
        }
        None
    })
}

fn exercise5_4(expected_hash: &[u8]) -> Option<usize> {
    (1usize..100_000)
        .into_iter()
        .find(|m| sha_512_n(&m.to_le_bytes(), 16).eq(expected_hash))
}

fn main() {
    for n in (2..4).step_by(8) {
        match exercise5_3(n) {
            Some((m1, m2)) => println!(
                "{} - {}{:?}, {}{:?}",
                n,
                m1,
                sha_512_n(&m1.to_le_bytes(), n),
                m2,
                sha_512_n(&m2.to_le_bytes(), n)
            ),
            None => println!("{} - No collision found!", n),
        };
    }

    let expected_hash = [0x3d, 0x4b];
    match exercise5_4(&expected_hash) {
        Some(message) => println!("{}{:?}", message, sha_512_n(&message.to_le_bytes(), 16)),
        None => println!("No match found for {:?}", expected_hash),
    }
}

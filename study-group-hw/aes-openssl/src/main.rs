use std::iter::repeat;

use openssl::{
    rsa::{Padding, Rsa},
    symm::{decrypt, encrypt, Cipher},
};

// https://www.openssl.org/docs/manmaster/man1/openssl-enc.html
// echo -e -n "\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07" |  openssl enc -d -aes-256-ecb -nopad -K "8000000000000000000000000000000000000000000000000000000000000001"
// -d : decrypt
// -K : key
// -nopad : disable block padding
fn question38() {
    let ciphertext = [
        0x53, 0x9B, 0x33, 0x3B, 0x39, 0x70, 0x6D, 0x14, 0x90, 0x28, 0xCF, 0xE1, 0xD9, 0xD4, 0xA4,
        0x07,
    ];

    let key = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];

    let cipher = Cipher::aes_256_cbc();

    let plaintext = decrypt(cipher, &key, None, &ciphertext).unwrap();

    println!("Question 3.8: {:?}", plaintext);
}

fn question39() {
    let plaintext = [
        0x29, 0x6C, 0x93, 0xFD, 0xF4, 0x99, 0xAA, 0xEB, 0x41, 0x94, 0xBA, 0xBC, 0x2E, 0x63, 0x56,
        0x1D,
    ];

    let key = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];

    let cipher = Cipher::aes_256_cbc();

    let data = encrypt(cipher, &key, None, &plaintext).unwrap();

    // println!("{:?}", data);

    let rsa = Rsa::generate(3072).unwrap();
    let padding = Padding::PKCS1;

    let mut to = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(&data, &mut to, padding).unwrap();

    let mut out = vec![0; rsa.size() as usize];
    rsa.private_decrypt(&to, &mut out, padding).unwrap();

    println!("Question 3.9: {:?}", out);
}

fn question310() {
    let complement = |arr: &[u8]| -> Vec<u8> { arr.iter().map(|b| !b).collect() };

    // DES 56 bit key, but it somehow takes 64 bits, go figure
    let key: [u8; 8] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];

    let plaintext: [u8; 8] = [
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
    ];

    let cipher = Cipher::des_cbc();

    let left = encrypt(cipher, &complement(&key), None, &complement(&plaintext)).unwrap();
    let right = complement(&encrypt(cipher, &key, None, &plaintext).unwrap());

    assert_eq!(left, right)
}

fn question44() {
    let key: [u8; 32] = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];

    let ciphertext_with_iv: [u8; 48] = [
        0x87, 0xF3, 0x48, 0xFF, 0x79, 0xB8, 0x11, 0xAF, 0x38, 0x57, 0xD6, 0x71, 0x8E, 0x5F, 0x0F,
        0x91, //
        0x7C, 0x3D, 0x26, 0xF7, 0x73, 0x77, 0x63, 0x5A, 0x5E, 0x43, 0xE9, 0xB5, 0xCC, 0x5D, 0x05,
        0x92, 0x6E, 0x26, 0xFF, 0xC5, 0x22, 0x0D, 0xC7, 0xD4, 0x05, 0xF1, 0x70, 0x86, 0x70, 0xE6,
        0xE0, 0x17,
    ];

    let cipher = Cipher::aes_256_cbc();

    let iv = ciphertext_with_iv[..16].to_owned();
    let ciphertext = ciphertext_with_iv[16..].to_owned();
    let plaintext = decrypt(cipher, &key, Some(&iv), &ciphertext).unwrap();

    println!("Question 4.4: {:?}", plaintext);
}

fn pkcs7(plaintext: Vec<u8>) -> Vec<u8> {
    assert!(plaintext.len().le(&255));

    let padding_byte = 255u8 - plaintext.len() as u8;

    let mut padded_plaintext = Vec::with_capacity(255);
    padded_plaintext.extend_from_slice(&plaintext);

    let padding = repeat(padding_byte).take(padding_byte as usize);
    padded_plaintext.extend(padding);

    padded_plaintext
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pkcs7() {
        let plaintext: [u8; 8] = [
            u8::MAX,
            u8::MAX,
            u8::MAX,
            u8::MAX,
            u8::MAX,
            u8::MAX,
            u8::MAX,
            u8::MAX,
        ];

        let padded = pkcs7(plaintext.into());
        // that's a bit dumb but meh
        assert_eq!(padded.len(), 255);
    }
}

fn main() {
    question38();
    question39();
    question310();
    // question44();
}

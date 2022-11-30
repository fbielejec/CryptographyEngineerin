use openssl::{
    rsa::{Padding, Rsa},
    symm::{decrypt, encrypt, Cipher},
};

// https://www.openssl.org/docs/manmaster/man1/openssl-enc.html
// echo -e -n "\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07" |  openssl enc -d -aes-256-ecb -nopad -K "8000000000000000000000000000000000000000000000000000000000000001"
// -d : decrypt
// -K : key
// -nopad : disable block padding
fn question4() {
    let ciphertext = [
        0x53, 0x9B, 0x33, 0x3B, 0x39, 0x70, 0x6D, 0x14, 0x90, 0x28, 0xCF, 0xE1, 0xD9, 0xD4, 0xA4,
        0x07,
    ];

    // println!("{:?}", &ciphertext);

    let key = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];

    let cipher = Cipher::aes_256_ecb();

    let plaintext = decrypt(cipher, &key, None, &ciphertext).unwrap();

    println!("{:?}", plaintext);
}

fn question9() {
    let plaintext = [
        0x29, 0x6C, 0x93, 0xFD, 0xF4, 0x99, 0xAA, 0xEB, 0x41, 0x94, 0xBA, 0xBC, 0x2E, 0x63, 0x56,
        0x1D,
    ];

    let key = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];

    let cipher = Cipher::aes_256_ecb();

    let data = encrypt(cipher, &key, None, &plaintext).unwrap();

    println!("{:?}", data);

    let rsa = Rsa::generate(3072).unwrap();
    let padding = Padding::PKCS1;

    let mut to = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(&data, &mut to, padding).unwrap();

    let mut out = vec![0; rsa.size() as usize];
    rsa.private_decrypt(&to, &mut out, padding).unwrap();

    println!("{:?}", out);
}

fn question10() {
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

fn main() {
    question4();
    question9();
    question10();
}

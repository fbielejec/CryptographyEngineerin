pub fn encrypt(message: &str, key: &str) -> String {
    (0..message.len())
        .map(|index| {
            let c = (key.chars().nth(index).unwrap().to_ascii_uppercase() as i32
                + message.chars().nth(index).unwrap().to_ascii_uppercase() as i32)
                % 26;

            alphabet_position_to_char((c + 1) as u8)
        })
        .collect()
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    (0..ciphertext.len())
        .map(|index| {
            let c = (ciphertext.chars().nth(index).unwrap().to_ascii_uppercase() as i32
                - key.chars().nth(index).unwrap().to_ascii_uppercase() as i32
                + 26)
                % 26;

            alphabet_position_to_char((c + 1) as u8)
        })
        .collect()
}

// This function generates the key in
// a cyclic manner until it's length is
// equal to the length of original text
pub fn generate_key(keyword: &str, length: usize) -> String {
    // let message = message.to_ascii_lowercase();
    let keyword = keyword.to_ascii_lowercase();
    (0..length)
        .map(|index| keyword.chars().nth(index % keyword.len()).unwrap())
        .collect()
}

fn alphabet_position_to_char(pos: u8) -> char {
    (pos + 64u8) as char
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vignere_cipher() {
        let message = "WHATANICEDAYTODAY";
        let keyword = "CRYPTO";

        let key = generate_key(keyword, message.len());

        let cipher_text = encrypt(message, &key);
        let decoded_text = decrypt(&cipher_text, &key);

        assert_eq!(decoded_text, message);
    }
}

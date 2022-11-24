pub use anyhow::Result as AnyResult;
use rand::rngs::ThreadRng;
use rsa::{errors::Error as RSAError, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

#[derive(Debug, Clone)]
pub struct RSA {
    pub rng: ThreadRng,
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

#[derive(Debug)]
pub enum Error {
    RSAError(RSAError),
    EncryptionError,
    DecryptionError,
}

impl Default for RSA {
    fn default() -> Self {
        let rng = rand::thread_rng();
        let bit_size = 1024;
        RSA::new(rng, bit_size).unwrap()
    }
}

impl RSA {
    pub fn new(mut rng: ThreadRng, bit_size: usize) -> Result<Self, Error> {
        let private_key = RsaPrivateKey::new(&mut rng, bit_size).map_err(Error::RSAError)?;

        let public_key = RsaPublicKey::from(&private_key);
        Ok(Self {
            rng,
            private_key,
            public_key,
        })
    }

    pub fn encrypt(&mut self, data: String) -> Result<Vec<u8>, RSAError> {
        self.public_key.encrypt(
            &mut self.rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            data.as_bytes(),
        )
    }

    pub fn decrypt(&self, cyphertext: Vec<u8>) -> Result<String, Error> {
        String::from_utf8(
            self.private_key
                .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &cyphertext)
                .map_err(|_e| Error::DecryptionError)?,
        )
        .map_err(|_e| Error::DecryptionError)
    }
}

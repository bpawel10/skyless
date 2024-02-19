use openssl::{
    bn::BigNum,
    rsa::{Padding, Rsa as RsaKey, RsaPrivateKeyBuilder},
};
use std::{env::var, io::Result};

const PADDING: usize = 128;

#[derive(Debug)]
pub struct Rsa {
    d: String,
    e: String,
    n: String,
}

impl Rsa {
    pub fn new() -> Self {
        Self {
            d: var("RSA_D").unwrap(),
            e: var("RSA_E").unwrap(),
            n: var("RSA_N").unwrap(),
        }
    }

    fn d(&self) -> Result<BigNum> {
        Ok(BigNum::from_dec_str(&self.d)?)
    }

    fn e(&self) -> Result<BigNum> {
        Ok(BigNum::from_dec_str(&self.e)?)
    }

    fn n(&self) -> Result<BigNum> {
        Ok(BigNum::from_dec_str(&self.n)?)
    }

    pub fn encrypt(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let padding = PADDING - buffer.len();
        let mut to_encrypt = buffer.to_vec();
        to_encrypt.append(&mut vec![0; padding]);
        let mut encrypted = vec![0; to_encrypt.len()];

        RsaKey::from_public_components(self.n()?, self.e()?)?.public_encrypt(
            &to_encrypt,
            &mut encrypted,
            Padding::NONE,
        )?;

        Ok(encrypted)
    }

    pub fn decrypt(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut decrypted = vec![0; PADDING];

        RsaPrivateKeyBuilder::new(self.n()?, self.e()?, self.d()?)?
            .build()
            .private_decrypt(buffer, &mut decrypted, Padding::NONE)?;

        Ok(decrypted)
    }
}

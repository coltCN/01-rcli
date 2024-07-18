use std::{fs, io::Read, path::Path};

use anyhow::{Ok, Result};
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{cli::TextSignFormat, get_reader};

use super::gen_pass;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(sign == hash)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Blake3::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = gen_pass::process_genpass(true, true, true, true, 32)?;
        let key = key.as_bytes();
        Ok(vec![key.to_vec()])
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Blake3::new(key))
    }
}
pub struct Ed25519Signer {
    key: SigningKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = self.key.sign(&buf);

        Ok(sign.to_vec())
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let vkey = signing_key.verifying_key().to_bytes().to_vec();
        let pkey = signing_key.to_bytes().to_vec();

        Ok(vec![pkey, vkey])
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sign.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Ed25519Verifier {
    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let verifyier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key.as_bytes())?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key.as_bytes())?),
    };
    verifyier.verify(&mut reader, sig.as_bytes())
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::{Blake3, TextSign};

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&mut data.as_ref(), &sig)?);
        Ok(())
    }
}

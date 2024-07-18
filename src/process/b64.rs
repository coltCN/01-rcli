use base64::prelude::{Engine as _, BASE64_STANDARD, BASE64_URL_SAFE};

use crate::{cli::Base64Format, get_reader};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&buf),
    };

    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // 避免多余的换行符
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(buf)?,
    };

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};

    use super::*;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        // assert!(process_decode(input, format).is_ok());
        process_decode(input, format)?;
        Ok(())
    }
}

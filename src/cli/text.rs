use core::fmt;
use std::str::FromStr;

use anyhow::Ok;
use clap::Parser;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a pivate/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a sign message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long,value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long,value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long)]
    pub sign: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

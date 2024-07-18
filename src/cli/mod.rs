mod base64;
mod csv;
mod genpass;
mod text;

use self::csv::CsvOpts;
pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    text::{TextSignFormat, TextSubCommand},
};
use clap::Parser;
use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[clap(name = "rcli", version, author)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),
}
fn verify_file(file_name: &str) -> Result<String, &'static str> {
    // 判断 file_name 是 "-" 或者 文件是否存在
    if file_name == "-" || std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}

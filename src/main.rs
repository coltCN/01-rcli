// rcli csv -i input.csv -o output.json --heard -d ','

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify, Base64SubCommand, Opts, SubCommand, TextSubCommand,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
                opts.length,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                // TODO: decode 结果 不一定是String，但是这里 假设是
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, &opts.sign, opts.format)?;
            }
        },
    }

    Ok(())
}

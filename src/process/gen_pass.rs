use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

use crate::opts::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(opts: GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("upper won't empty!"));
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("lower won't empty!"));
    }

    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("number won't empty!"));
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("symbol won't empty!"));
    }

    for _ in 0..(opts.length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't empty");
        password.push(*c)
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[]);
    eprintln!("Password strength: {}", estimate.score());
    Ok(())
}

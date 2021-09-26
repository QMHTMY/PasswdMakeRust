use anyhow::{Result, bail};
use structopt::StructOpt;
use encryptor::password::generate_password;

/// 命令行参数结构体
#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short="s", long="seed")]
    seed: String,

    #[structopt(short="l", long="len", default_value = "16")]
    length: usize,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    if args.seed.len() < 4 {
        bail!("seed `{}` length must >= 4", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);
    let passwd = generate_password(&seed[..], length);
    match passwd {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }

    Ok(())
}

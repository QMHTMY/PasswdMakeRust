extern crate base64;

use base64::encode;
use anyhow::{Result, bail};
use structopt::StructOpt;
use hash::merhash::mersenne_hash;

/// 命令行参数结构体
#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short="s", long="seed")]
    seed: String,

    #[structopt(short="l", long="length")]
    length: usize,
}

/// 密码子 (长度 100)，可随意交换次序，增减字符，实现个性化定制
const CRYPTO:&str = "!pqHr$*+STKU1%Vst_uv:w{WSX&YZ-/01_2.34<ABECo|x#yDE^FG?HEI[]JK>LM#NOBWPQ:Ra@}cde56R7=8l9f/9gIhi,jkzmn";

/// 哈希密码函数 hash_password，利用哈希值的高次方来映射密码子中的字符作为密码
///
/// #Example
/// ```
/// let seed = "jdxjp".to_string();
/// let length = 16;
/// let passwd = hash_password(&seed, length);
/// println!("{}",passwd);
/// ```
fn hash_password(seed: &String, length: usize) -> String {
    // 判断秘密长度并决定哈希值的次方数 p
    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut mer_hash = mersenne_hash(&seed).pow(p);

    // 由 mer_hash 计算 passwd
    let mut passwd = String::new();
    while mer_hash > 9 {
        let loc = mer_hash % CRYPTO.len();
        passwd.push(CRYPTO.chars().nth(loc).unwrap());
        mer_hash = (mer_hash / CRYPTO.len()) as usize;
    }
    
    // 由 seed 拼接其字符到 passwd
    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }

    // 将 passwd encode 为 base64 截取 base64 中前 length 个字符，替换其中的 + 和 / 返回最终结果作为 passwd
    passwd = encode(&passwd.as_bytes());
    passwd = seed.to_owned() + ": " + &passwd[..length];
    passwd = passwd.replace("+","*").replace("/","_");
    passwd
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    if args.seed.len() < 4 {
        bail!("seed = {}, its length must >= 4", &args.seed);
    }

    let passwd = hash_password(&args.seed, args.length);
    println!("{}", passwd);

    Ok(())
}

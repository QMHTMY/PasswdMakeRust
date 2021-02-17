extern crate base64;

use std::env;
use base64::encode;
use hash::merhash::mersenne_hash;

/// 密码子 (长度 100)
const CRYPTO:&str = "!pqHr$*+STKU1%Vst_uv:w{WSX&YZ-/01_2.34<ABECo|x#yDE^FG?HEI[]JK>LM#NOBWPQ:Ra@}cde56R7=8l9f/9gIhi,jkzmn";

/// 哈希密码函数 hash_password
/// 利用哈希值的高次方来映射密码子中的字符
/// 组合的字符再和 seed 拼接并编码为 base64
/// 取合适长度的字符串作为最终密码
///
/// #Example
/// ```
/// let seed = "jdxjp".to_string();
/// let length = 16;
/// let passwd = hash_password(&seed, length);
/// println!("{}",passwd);
/// ```
fn hash_password(seed: &String, length: usize) -> String {
    // 1.由 seed 计算 mer_hash
    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut mer_hash = mersenne_hash(&seed).pow(p);

    // 2.由 mer_hash 计算 passwd
    let mut passwd = String::new();
    while mer_hash > 9 {
        let loc = mer_hash % 100;
        passwd.push(CRYPTO.chars().nth(loc).unwrap());
        mer_hash = (mer_hash / 100) as usize;
    }
    
    // 3.由 seed 拼接其字符到 passwd
    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }

    // 4.1将 passwd encode 为 base64 
    // 4.2截取 base64 中前 length 个字符，替换其中的 + 和 /
    // 4.3返回最终结果作为 passwd
    passwd = encode(&passwd.as_bytes());
    passwd = seed.to_owned() + ": " + &passwd[..length];
    passwd = passwd.replace("+","*").replace("/","_");
    passwd
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        let paths: Vec<&str> = args[0].split('/').collect();
        let log_info = format!("Usage: {} seed [length]", paths[paths.len()-1]);
        Err(log_info)?;
    } 

    let (seed, length) = if args.len() == 2 {
        (&args[1], 16) // 密码默认长度16
    } else {
        (&args[1], args[2].parse::<usize>().unwrap())
    };

    if seed.len() < 4 {
        let log_info = format!("Error: seed = {}, its length less than 4", seed);
        Err(log_info)?;
    }

    let mut passwd = String::new();
    passwd = hash_password(seed, length);
    println!("{}",passwd);

    Ok(())
}

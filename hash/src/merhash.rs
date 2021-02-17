/// 梅森哈希函数 mersenne_hash 
/// 用梅森素数 127 计算哈希值
/// 然后求哈希值的三次方减1
///
/// # Example
/// ```
/// let seed = "jdxjp".to_string(); 
/// let hash = mersenne_hash(&seed);
/// println!("{}",hash);
/// ```
pub fn mersenne_hash(seed: &String) -> usize {
    let mut hash: usize = 0;

    for (i, c) in seed.chars().enumerate() {
        hash += (i+1) * (c as usize);
    }

    (hash % 127).pow(3) - 1
}

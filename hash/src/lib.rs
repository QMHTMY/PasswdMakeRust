pub mod merhash;

#[cfg(test)]
mod tests {
    use crate::merhash::mersenne_hash;

    #[test]
    fn mersenne_hash_works() {
        let seed = "jdxjp".to_string();
        let hash = mersenne_hash(&seed);
        if hash > 0 {
            assert_eq!(2 + 2, 4);
        } else {
            assert_eq!(2 + 2, 3);
        }
    }
}

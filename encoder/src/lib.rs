pub mod base58;

#[cfg(test)]
mod tests {
    use crate::base58::{Encoder, Decoder};

    #[test]
    fn encode_decode() {
        assert_eq!("ZiCa","abc".encode());
        assert_eq!("我爱你iloveu","7T5VrPqoBr9DeUXiUr2Fn".decode().unwrap());
    }
}

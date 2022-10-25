use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn get_signature(key: &str, message: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();

    mac.update(message.as_bytes());

    let result = mac.finalize();

    let result = result.into_bytes();
    let bytes = result.as_slice();

    let signature = hex::encode(bytes);

    signature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let key = "my secret and secure key";
        let message = "hello world";

        assert_eq!(
            get_signature(key, message),
            "0b6594ce57238a3c2473b0dfb9d700398044f7c43246dce5303bbc4a79fb24d3"
        )
    }
}
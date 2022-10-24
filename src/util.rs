use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn get_signature(key: &str, query_str: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();

    mac.update(query_str.as_bytes());

    let result = mac.finalize();

    let result = result.into_bytes();
    let bytes = result.as_slice();

    let signature = hex::encode(bytes);

    signature
}

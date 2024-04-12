use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref KEYS: Keys = Keys::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes());
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
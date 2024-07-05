use lazy_static::lazy_static;
use jsonwebtoken::{ EncodingKey, DecodingKey };

lazy_static! {
    /// Secret key for generating JWT encoding and decoding keys.
    static ref SECRET_KEY: String = "isaac-fei".to_string();

    /// JWT encoding key.
    pub static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SECRET_KEY.as_ref());

    /// JWT decoding key.
    pub static ref JWT_DECODING_KEY: DecodingKey = DecodingKey::from_secret(SECRET_KEY.as_ref());
}

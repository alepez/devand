use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Expiration time (as UTC timestamp)
    sub: String,
}

pub struct SignedToken(String);

pub struct Encoder {
    encoding_key: EncodingKey,
}

impl Encoder {
    pub fn new_from_secret(secret: &[u8]) -> Self {
        let encoding_key = EncodingKey::from_secret(secret);
        Self { encoding_key }
    }

    fn encode_claims(&self, claims: Claims) -> Option<SignedToken> {
        encode(&Header::default(), &claims, &self.encoding_key)
            .ok()
            .map(|x| SignedToken(x))
    }

    pub fn encode<T>(&self, data: &T) -> Option<SignedToken>
    where
        T: serde::ser::Serialize,
    {
        let encoded_data = serde_json::to_string(data).ok()?;
        let claims = Claims {
            exp: 9999999999,
            sub: encoded_data,
        };
        self.encode_claims(claims)
    }
}

pub struct Decoder<'a> {
    decoding_key: DecodingKey<'a>,
}

impl<'a> Decoder<'a> {
    pub fn new_from_secret(secret: &'a [u8]) -> Self {
        let decoding_key = DecodingKey::from_secret(&secret);
        Self { decoding_key }
    }

    fn decode_claims(&self, token: &SignedToken) -> Option<Claims> {
        let validation = Validation::default();
        decode::<Claims>(&token.0, &self.decoding_key, &validation)
            .map(|x| x.claims)
            .ok()
    }

    pub fn decode<T>(&self, token: &SignedToken) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let decoded: Claims = self.decode_claims(token)?;
        serde_json::from_str(&decoded.sub).ok()?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_decode() {
        let key = b"secret";
        let encoder = Encoder::new_from_secret(key);
        let decoder = Decoder::new_from_secret(key);

        let sub = "example@example.com";

        let claims = Claims {
            sub: sub.to_string(),
            exp: 9999999999,
        };

        let token = encoder.encode_claims(claims).unwrap();
        let data = decoder.decode_claims(&token).unwrap();

        assert!(sub == &data.sub);
    }

    #[test]
    fn encode_decode_generic() {
        let key = b"secret";
        let encoder = Encoder::new_from_secret(key);
        let decoder = Decoder::new_from_secret(key);

        #[derive(Serialize, Deserialize, Debug)]
        struct Data {
            x: u32,
        };

        let data = Data { x: 42 };

        let token = encoder.encode(&data).unwrap();
        let decoded: Data = decoder.decode(&token).unwrap();

        assert_eq!(decoded.x, data.x);
    }
}

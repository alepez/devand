use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub trait Signable {
    const EXP_SECONDS: i64;
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Expiration time (as UTC timestamp)
    sub: String,
}

#[derive(Clone)]
pub(crate) struct SignedToken(String);

impl Into<String> for SignedToken {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for SignedToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

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

    pub(crate) fn encode<T>(&self, data: &T) -> Option<SignedToken>
    where
        T: serde::ser::Serialize + Signable,
    {
        let encoded_data = serde_json::to_string(data).ok()?;
        let now = chrono::Utc::now();
        let exp = now.checked_add_signed(chrono::Duration::seconds(T::EXP_SECONDS))?;
        let exp = exp.timestamp() as usize;
        let claims = Claims {
            exp,
            sub: encoded_data,
        };
        self.encode_claims(claims)
    }
}

// Note: we cannot store DecodingKey here
// DecodingKey has a lifetime parameter and Decoder::new() cannot
// return a DecodingKey with reference to the secret.
// See https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
pub struct Decoder {
    secret: Vec<u8>,
}

impl Decoder {
    pub fn new_from_secret(secret: &[u8]) -> Self {
        let secret = secret.to_owned();
        Self { secret }
    }

    fn decode_claims(&self, token: &SignedToken) -> Option<Claims> {
        let decoding_key = DecodingKey::from_secret(&self.secret);
        let validation = Validation::default();
        decode::<Claims>(&token.0, &decoding_key, &validation)
            .map(|x| x.claims)
            .ok()
    }

    pub(crate) fn decode<T>(&self, token: &SignedToken) -> Option<T>
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

        impl Signable for Data {
            const EXP_SECONDS: i64 = 3600;
        }

        let data = Data { x: 42 };

        let token = encoder.encode(&data).unwrap();
        let decoded: Data = decoder.decode(&token).unwrap();

        assert_eq!(decoded.x, data.x);
    }
}

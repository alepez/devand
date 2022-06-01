use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Implement this trait for all types that needs to be converted to a SignedToken
pub trait Signable {
    /// Identifies the expiration (in seconds since encoding) after which the
    /// JWT must not be accepted for processing.
    const EXP_SECONDS: i64;

    fn sign(&self, encoder: &Encoder) -> SignedToken
    where
        Self: serde::ser::Serialize + std::marker::Sized,
    {
        encoder.encode(self).expect("Token is encoded")
    }

    fn try_from_token(token: &SignedToken, decoder: &Decoder) -> Option<Self>
    where
        Self: serde::de::DeserializeOwned + std::marker::Sized,
    {
        decoder.decode(token)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Expiration time (as UTC timestamp)
    sub: String,
}

#[derive(Clone)]
pub struct SignedToken(String);

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

impl std::fmt::Display for SignedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Encoder {
    encoding_key: EncodingKey,
}

impl Encoder {
    /// Creates an HMAC encoder from a secret key
    pub fn new_from_secret(secret: &[u8]) -> Self {
        let encoding_key = EncodingKey::from_secret(secret);
        Self { encoding_key }
    }

    fn encode_claims(&self, claims: Claims) -> Option<SignedToken> {
        encode(&Header::default(), &claims, &self.encoding_key)
            .ok()
            .map(SignedToken)
    }

    /// Encode any Signable type
    pub(crate) fn encode<T>(&self, data: &T) -> Option<SignedToken>
    where
        T: serde::ser::Serialize + Signable,
    {
        let encoded_data = bincode::serialize(data).ok()?;
        let base64_encoded_data = base64::encode(&encoded_data);
        let now = chrono::Utc::now();
        let exp = now.checked_add_signed(chrono::Duration::seconds(T::EXP_SECONDS))?;
        let exp = exp.timestamp() as usize;
        let claims = Claims {
            exp,
            sub: base64_encoded_data,
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
    /// Creates an HMAC decoder from a secret key
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

    /// Decode any deserializable type from a SignedToken
    pub(crate) fn decode<T>(&self, token: &SignedToken) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let decoded: Claims = self.decode_claims(token)?;
        let base64_encoded_data = decoded.sub;
        let encoded_data = base64::decode(base64_encoded_data).ok()?;
        bincode::deserialize(&encoded_data).ok()
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

        let token = data.sign(&encoder);
        let decoded = Data::try_from_token(&token, &decoder).unwrap();

        assert_eq!(decoded.x, data.x);
    }
}

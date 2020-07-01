use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Expiration time (as UTC timestamp)
    sub: String,
}

struct Token(String);

struct Encoder {
    encoding_key: EncodingKey,
}

impl Encoder {
    fn new_from_secret(secret: &[u8]) -> Self {
        let encoding_key = EncodingKey::from_secret(secret);
        Self { encoding_key }
    }

    fn encode(&self, claims: Claims) -> Option<Token> {
        encode(&Header::default(), &claims, &self.encoding_key)
            .ok()
            .map(|x| Token(x))
    }
}

struct Decoder<'a> {
    decoding_key: DecodingKey<'a>,
}

impl<'a> Decoder<'a> {
    fn new_from_secret(secret: &'a [u8]) -> Self {
        let decoding_key = DecodingKey::from_secret(&secret);
        Self { decoding_key }
    }

    fn decode(&self, token: Token) -> Option<Claims> {
        let validation = Validation::default();
        decode::<Claims>(&token.0, &self.decoding_key, &validation)
            .map(|x| x.claims)
            .ok()
    }
}

struct DataVerifier<'a> {
    encoder: Encoder,
    decoder: Decoder<'a>,
}

impl<'a> DataVerifier<'a> {
    fn new_from_secret(secret: &'a [u8]) -> Self {
        Self {
            encoder: Encoder::new_from_secret(secret),
            decoder: Decoder::new_from_secret(secret),
        }
    }

    fn encode<T>(&self, data: &T) -> Option<Token>
    where
        T: serde::ser::Serialize,
    {
        let encoded_data = serde_json::to_string(data).ok()?;
        let claims = Claims {
            exp: 9999999999,
            sub: encoded_data,
        };
        self.encoder.encode(claims)
    }

    fn decode<T>(&self, token: Token) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let decoded: Claims = self.decoder.decode(token)?;
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

        let token = encoder.encode(claims).unwrap();
        let data = decoder.decode(token).unwrap();

        assert!(sub == &data.sub);
    }

    #[test]
    fn verifier() {
        let key = b"secret";
        let verifier = DataVerifier::new_from_secret(key);

        #[derive(Serialize, Deserialize, Debug)]
        struct Data {
            x: u32,
        };

        let data = Data { x: 42 };

        let token = verifier.encode(&data).unwrap();
        let decoded: Data = verifier.decode(token).unwrap();

        assert_eq!(decoded.x, data.x);
    }
}

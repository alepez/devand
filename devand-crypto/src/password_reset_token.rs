use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // aud: String, // Optional. Audience
    exp: usize, // Expiration time (as UTC timestamp)
    // iat: usize,  // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize,  // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
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
}

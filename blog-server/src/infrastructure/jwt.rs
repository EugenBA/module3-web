
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey,
                   errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use anyhow::Result;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i64,
    username: String,
    exp: i64,
}

#[derive(Debug)]
struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtService {
    fn new(secret: &str) -> Self {
        JwtService {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    fn generate_token(&self, user_id: i64, username: &str) -> Result<String, JwtError> {
        let claims = Claims {
            user_id,
            username: username.to_string(),
            exp: (Utc::now() +
                Duration::hours(24)).timestamp()
        };
        encode(&Header::default(), &claims, &self.encoding)
    }

    fn verify_token(&self, token: &str) -> Result<Claims, JwtError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding,
            &Validation::default()
        )?;
        if token_data.claims.exp < Utc::now().timestamp() {
            return Err(JwtError::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature));
        }
        Ok(token_data.claims)
    }
}
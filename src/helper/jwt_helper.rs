use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use sha2::Sha256;
use thiserror::Error;

use crate::JwtConfig;

#[derive(Debug, Error)]
pub enum JwtKind {
    #[error("InvalidKey")]
    InvalidKey,
    #[error("SignFailed")]
    SignFailed,
    #[error("MissingSubject")]
    MissingSubject,
}

#[derive(Clone)]
pub struct JwtHelper {
    issuer: String,
    key: Hmac<Sha256>,
}

impl JwtHelper {
    pub fn from_config(config: &JwtConfig) -> Self {
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.secret.as_bytes()).unwrap();

        JwtHelper {
            issuer: config.issuer.clone(),
            key,
        }
    }

    pub fn encode(&self, raw_data: &str) -> Result<String, JwtKind> {
        let claims = RegisteredClaims {
            issuer: Some(self.issuer.clone().into()),
            subject: Some(raw_data.into()),
            ..Default::default()
        };

        let signed_token = claims
            .sign_with_key(&self.key)
            .map_err(|_e| JwtKind::SignFailed)?;

        Ok(signed_token)
    }

    pub fn decode(&self, token: &str) -> Result<String, JwtKind> {
        let claims: RegisteredClaims =
            VerifyWithKey::verify_with_key(token, &self.key).map_err(|_e| JwtKind::SignFailed)?;

        claims.subject.ok_or(JwtKind::MissingSubject)
    }
}

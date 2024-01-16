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

pub struct JwtHelper {
    issuer: String,
    secret: String,
}

impl JwtHelper {
    pub fn from_config(config: &JwtConfig) -> Self {
        JwtHelper {
            issuer: config.issuer.clone(),
            secret: config.secret.clone(),
        }
    }

    pub fn encode(&self, raw_data: &str) -> Result<String, JwtKind> {
        let claims = RegisteredClaims {
            issuer: Some(self.issuer.clone().into()),
            subject: Some(raw_data.into()),
            ..Default::default()
        };

        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|_e| JwtKind::InvalidKey)?;

        let signed_token = claims
            .sign_with_key(&key)
            .map_err(|_e| JwtKind::SignFailed)?;

        Ok(signed_token)
    }

    pub fn decode(&self, token: &str) -> Result<String, JwtKind> {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|_e| JwtKind::InvalidKey)?;
        let claims: RegisteredClaims =
            VerifyWithKey::verify_with_key(token, &key).map_err(|_e| JwtKind::SignFailed)?;

        claims.subject.ok_or(JwtKind::MissingSubject)
    }
}

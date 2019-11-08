use crate::models::Request;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    Password,
    ClientCredentials,
    ClientSignature,
    RefreshToken,
}

impl Default for GrantType {
    fn default() -> GrantType {
        GrantType::Password
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AuthRequest {
    pub grant_type: GrantType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl AuthRequest {
    pub fn credential_auth(key: &str, secret: &str) -> AuthRequest {
        AuthRequest {
            grant_type: GrantType::ClientCredentials,
            client_id: Some(key.into()),
            client_secret: Some(secret.into()),
            ..Default::default()
        }
    }

    pub fn password_auth(username: &str, password: &str) -> AuthRequest {
        AuthRequest {
            grant_type: GrantType::Password,
            username: Some(username.into()),
            password: Some(password.into()),
            ..Default::default()
        }
    }

    pub fn signature_auth(key: &str, timestamp: &str, signature: &str) -> AuthRequest {
        AuthRequest {
            grant_type: GrantType::ClientSignature,
            client_id: Some(key.into()),
            timestamp: Some(timestamp.into()),

            signature: Some(signature.into()),
            ..Default::default()
        }
    }

    pub fn refresh_token_auth(refresh_token: &str) -> AuthRequest {
        AuthRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: Some(refresh_token.into()),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthResponse {
    acccess_token: Option<String>,
    expires_in: i64,
    refresh_token: String,
    scope: String,
    state: Option<String>,
    token_type: String,
}

impl Request for AuthRequest {
    const METHOD: &'static str = "public/auth";
    type Response = AuthResponse;
}

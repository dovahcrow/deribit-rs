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
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl AuthRequest {
    pub fn credential_auth(id: &str, secret: &str) -> AuthRequest {
        AuthRequest {
            grant_type: GrantType::ClientCredentials,
            client_id: Some(id.into()),
            client_secret: Some(secret.into()),
            ..Default::default()
        }
    }

    pub fn signature_auth(
        id: &str,
        timestamp: &str,
        signature: &str,
        nonce: Option<&str>,
        data: Option<&str>,
    ) -> AuthRequest {
        AuthRequest {
            grant_type: GrantType::ClientSignature,
            client_id: Some(id.into()),
            timestamp: Some(timestamp.into()),
            signature: Some(signature.into()),
            nonce: nonce.map(|x| x.into()),
            data: data.map(|x| x.into()),
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExchangeTokenRequest {
    refresh_token: String,
    subject_id: i64,
}

impl Request for ExchangeTokenRequest {
    const METHOD: &'static str = "public/exchange_token";
    type Response = AuthResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForkTokenRequest {
    refresh_token: String,
    session_name: String,
}

impl Request for ForkTokenRequest {
    const METHOD: &'static str = "public/fork_token";
    type Response = AuthResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogoutRequest;

impl Request for LogoutRequest {
    const METHOD: &'static str = "private/logout";
    const HAS_PAYLOAD: bool = false;
    type Response = ();
}

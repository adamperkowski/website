use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use hex::decode;
use hmac::{digest::generic_array::GenericArray, Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

use crate::sse::SseState;

#[derive(Serialize, Deserialize)]
pub struct WebhookPayload {
    action: String,
    repository: Repository,
    sender: User,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    full_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    login: String,
}

type HmacSha256 = Hmac<Sha256>;

pub async fn handler(
    State(sse_state): State<Arc<SseState>>,
    headers: HeaderMap,
    payload: String,
) -> impl IntoResponse {
    let secret = std::env::var("GITHUB_SECRET").expect("GITHUB_SECRET not set (somehow?)");

    if !headers.contains_key("X-Hub-Signature-256") {
        return (
            StatusCode::BAD_REQUEST,
            "missing X-Hub-Signature-256 header".to_string(),
        )
            .into_response();
    }

    if let Err(e) = verify_signature(secret.as_bytes(), &headers["X-Hub-Signature-256"], &payload) {
        return (
            StatusCode::FORBIDDEN,
            format!("signature verification failed: {}", e),
        )
            .into_response();
    }

    let payload: WebhookPayload = match serde_json::from_str(&payload) {
        Ok(payload) => payload,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, format!("invalid payload: {}", e)).into_response();
        }
    };

    if payload.action != "created" {
        return (StatusCode::OK, "ignoring action != created".to_string()).into_response();
    }

    let message = format!(
        "{} just starred {}",
        payload.sender.login, payload.repository.full_name
    );

    println!("github: {message}");

    let _ = sse_state.tx.send(message);

    (StatusCode::OK, "ok".to_string()).into_response()
}

fn verify_signature(
    secret: &[u8],
    signature: &HeaderValue,
    payload: &String,
) -> Result<(), String> {
    let signature_bytes = decode(
        signature
            .to_str()
            .map_err(|_| "invalid header value")?
            .trim_start_matches("sha256="),
    )
    .map_err(|_| "invalid hex in signature")?;

    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| "invalid secret")?;
    mac.update(payload.as_bytes());

    Ok(mac
        .verify(GenericArray::from_slice(&signature_bytes))
        .map_err(|_| "signature mismatch")?)
}

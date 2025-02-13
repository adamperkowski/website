//! Oh come on, not this shit again...
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

use crate::constants;
use crate::sse::SseState;

#[derive(Serialize, Deserialize)]
pub struct StarPayload {
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

#[derive(Serialize, Deserialize)]
pub struct SponsorshipPayload {
    action: String,
    sponsorship: Sponsorship,
}

#[derive(Serialize, Deserialize)]
pub struct Sponsorship {
    sponsor: User,
    privacy_level: String,
    tier: SponsorshipTier,
}

#[derive(Serialize, Deserialize)]
pub struct SponsorshipTier {
    is_one_time: bool,
}

type HmacSha256 = Hmac<Sha256>;

pub async fn handler(
    State(sse_state): State<Arc<SseState>>,
    headers: HeaderMap,
    payload: String,
) -> impl IntoResponse {
    let secret = std::env::var(constants::GITHUB_SECRET_VAR).unwrap();

    if !headers.contains_key("X-Hub-Signature-256") || !headers.contains_key("X-GitHub-Event") {
        return (
            StatusCode::BAD_REQUEST,
            "missing required headers".to_string(),
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

    let message = match match headers["X-GitHub-Event"].to_str().unwrap_or_default() {
        "ping" => return (StatusCode::OK, "pong".to_string()).into_response(),
        "star" => handle_star(&payload).await,
        "sponsorship" => handle_sponsorship(&payload).await,
        _ => Err((StatusCode::BAD_REQUEST, "unknown event".to_string())),
    } {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };

    if let Err(e) = sse_state.tx.send(message) {
        println!("failed to send to SSE: {e}")
    };

    (StatusCode::OK, "ok".to_string()).into_response()
}

async fn handle_star(payload: &str) -> Result<String, (StatusCode, String)> {
    let payload: StarPayload = match serde_json::from_str(payload) {
        Ok(payload) => payload,
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("invalid payload: {e}")));
        }
    };

    if payload.action != "created" {
        return Err((StatusCode::OK, "ignoring action != created".to_string()));
    }

    println!(
        "github: {} just starred {}",
        payload.sender.login, payload.repository.full_name
    );

    Ok(format!(
        "<a href='https://github.com/{0}' target='_blank'>{0}</a> just starred <a href='https://github.com/{1}' target='_blank'>{1}</a>!",
        payload.sender.login, payload.repository.full_name
    ))
}

async fn handle_sponsorship(payload: &str) -> Result<String, (StatusCode, String)> {
    let payload: SponsorshipPayload = match serde_json::from_str(payload) {
        Ok(payload) => payload,
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("invalid payload: {e}")));
        }
    };

    if payload.action != "created" {
        return Err((StatusCode::OK, "ignoring action != created".to_string()));
    }

    if payload.sponsorship.privacy_level == "SECRET" {
        return Err((StatusCode::OK, "ignoring SECRET sponsorship".to_string()));
    }

    let tier = if payload.sponsorship.tier.is_one_time {
        "one-time donation"
    } else {
        "sponsorship subscription"
    };

    println!(
        "github: thanks for the {1}, {0}",
        payload.sponsorship.sponsor.login, tier
    );

    Ok(format!(
        "Thanks for the <a href='https://github.com/sponsors/adamperkowski' target='_blank'>{1}</a>, <a href='https://github.com/{0}' target='_blank'>{0}</a>!",
        payload.sponsorship.sponsor.login, tier
    ))
}

/// I'm sorry but why the FUCK is there no crate I can use for this??
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

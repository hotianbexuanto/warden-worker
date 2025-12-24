use axum::{extract::State, Json};
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use worker::{query, Env};

use crate::{
    auth::Claims,
    db,
    error::AppError,
    models::user::{PreloginResponse, RegisterRequest, User},
};

#[worker::send]
pub async fn prelogin(
    State(env): State<Arc<Env>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<PreloginResponse>, AppError> {
    let email = payload["email"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("Missing email".to_string()))?;
    let db = db::get_db(&env)?;

    let stmt = db.prepare("SELECT kdf_iterations FROM users WHERE email = ?1");
    let query = stmt.bind(&[email.into()])?;
    let kdf_iterations: Option<i32> = query
        .first(Some("kdf_iterations"))
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json(PreloginResponse {
        kdf: 0, // PBKDF2
        kdf_iterations: kdf_iterations.unwrap_or(600_000),
    }))
}

#[worker::send]
pub async fn register(
    State(env): State<Arc<Env>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<Value>, AppError> {
    let allowed_emails = env
        .secret("ALLOWED_EMAILS")
        .map_err(|_| AppError::Internal)?;
    let allowed_emails = allowed_emails
        .as_ref()
        .as_string()
        .ok_or_else(|| AppError::Internal)?;
    if allowed_emails
        .split(",")
        .all(|email| email.trim() != payload.email)
    {
        return Err(AppError::Unauthorized("Not allowed to signup".to_string()));
    }
    let db = db::get_db(&env)?;
    let now = Utc::now().to_rfc3339();
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        avatar_color: None,
        email: payload.email.to_lowercase(),
        email_verified: false,
        master_password_hash: payload.master_password_hash,
        master_password_hint: payload.master_password_hint,
        key: payload.user_symmetric_key,
        private_key: payload.user_asymmetric_keys.encrypted_private_key,
        public_key: payload.user_asymmetric_keys.public_key,
        kdf_type: payload.kdf,
        kdf_iterations: payload.kdf_iterations,
        security_stamp: Uuid::new_v4().to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    let query = query!(
        &db,
        "INSERT INTO users (id, name, avatar_color, email, email_verified, master_password_hash, master_password_hint, key, private_key, public_key, kdf_type, kdf_iterations, security_stamp, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
         user.id,
         user.name,
         user.avatar_color,
         user.email,
         user.email_verified,
         user.master_password_hash,
         user.master_password_hint,
         user.key,
         user.private_key,
         user.public_key,
         user.kdf_type,
         user.kdf_iterations,
         user.security_stamp,
         user.created_at,
         user.updated_at
    ).map_err(|error|{
        AppError::Database
    })?
    .run()
    .await
    .map_err(|error|{
        AppError::Database
    })?;

    Ok(Json(json!({})))
}

#[worker::send]
pub async fn send_verification_email() -> String {
    "fixed-token-to-mock".to_string()
}

/// Get the user's vault revision date
/// Returns a Unix timestamp in milliseconds representing the last time the user's vault was updated
/// This is used by mobile clients to quickly check if they need to sync without downloading all data
#[worker::send]
pub async fn revision_date(
    claims: Claims,
    State(env): State<Arc<Env>>,
) -> Result<Json<i64>, AppError> {
    let db = db::get_db(&env)?;

    // Get the most recent timestamp from user, ciphers, and folders
    // This accurately reflects when the vault was last changed
    let user_updated: Option<String> = db
        .prepare("SELECT updated_at FROM users WHERE id = ?1")
        .bind(&[claims.sub.clone().into()])?
        .first(Some("updated_at"))
        .await
        .map_err(|_| AppError::Database)?;

    let cipher_updated: Option<String> = db
        .prepare("SELECT updated_at FROM ciphers WHERE user_id = ?1 ORDER BY updated_at DESC LIMIT 1")
        .bind(&[claims.sub.clone().into()])?
        .first(Some("updated_at"))
        .await
        .ok()
        .flatten();

    let folder_updated: Option<String> = db
        .prepare("SELECT updated_at FROM folders WHERE user_id = ?1 ORDER BY updated_at DESC LIMIT 1")
        .bind(&[claims.sub.clone().into()])?
        .first(Some("updated_at"))
        .await
        .ok()
        .flatten();

    // Find the most recent timestamp among all sources
    let timestamps = vec![user_updated, cipher_updated, folder_updated];
    let most_recent = timestamps
        .into_iter()
        .flatten()
        .filter_map(|ts| chrono::DateTime::parse_from_rfc3339(&ts).ok())
        .map(|dt| dt.timestamp_millis())
        .max()
        .unwrap_or_else(|| chrono::Utc::now().timestamp_millis());

    Ok(Json(most_recent))
}

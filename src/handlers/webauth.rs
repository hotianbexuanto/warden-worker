// WebAuthn stub handler
// Returns empty list as passkeys/FIDO2 feature is not supported

use axum::{extract::State, Json};
use serde_json::json;
use std::sync::Arc;
use worker::Env;

use crate::auth::Claims;
use crate::error::AppError;

#[worker::send]
pub async fn get_webauthn_credentials(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({
        "data": [],
        "object": "list",
        "continuationToken": null
    })))
}

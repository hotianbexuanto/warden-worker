// Emergency access stub handlers
// These endpoints return empty lists as emergency access feature is not supported

use axum::{extract::State, Json};
use serde_json::json;
use std::sync::Arc;
use worker::Env;

use crate::auth::Claims;
use crate::error::AppError;

#[worker::send]
pub async fn get_trusted_contacts(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({
        "data": [],
        "object": "list",
        "continuationToken": null
    })))
}

#[worker::send]
pub async fn get_granted_access(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({
        "data": [],
        "object": "list",
        "continuationToken": null
    })))
}

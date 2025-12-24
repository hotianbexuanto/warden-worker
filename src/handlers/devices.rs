// Device management stub handlers
// These endpoints return empty responses to avoid 404 errors with Bitwarden clients
// Actual device tracking is not implemented

use axum::{extract::State, Extension, Json};
use serde_json::json;
use std::sync::Arc;
use worker::Env;

use crate::auth::Claims;
use crate::error::AppError;

#[worker::send]
pub async fn get_devices(
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
pub async fn get_known_device(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!(false)))
}

#[worker::send]
pub async fn get_device(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({})))
}

#[worker::send]
pub async fn post_device_token(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({})))
}

#[worker::send]
pub async fn put_device_token(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({})))
}

#[worker::send]
pub async fn put_clear_device_token(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({})))
}

#[worker::send]
pub async fn post_clear_device_token(
    _claims: Claims,
    State(_env): State<Arc<Env>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({})))
}

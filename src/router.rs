use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use worker::Env;

use crate::handlers::{accounts, ciphers, config, identity, sync, folders, import, devices, emergency_access, webauth};

pub fn api_router(env: Env) -> Router {
    let app_state = Arc::new(env);

    Router::new()
        // Identity/Auth routes
        .route("/identity/accounts/prelogin", post(accounts::prelogin))
        .route(
            "/identity/accounts/register/finish",
            post(accounts::register),
        )
        .route("/identity/connect/token", post(identity::token))
        .route(
            "/identity/accounts/register/send-verification-email",
            post(accounts::send_verification_email),
        )
        // Main data sync route
        .route("/api/sync", get(sync::get_sync_data))
        // Revision date for on-demand sync checks (mobile refresh)
        .route("/api/accounts/revision-date", get(accounts::revision_date))
        // Ciphers CRUD
        .route("/api/ciphers", post(ciphers::create_cipher_standard))
        .route("/api/ciphers/create", post(ciphers::create_cipher))
        .route("/api/ciphers/import", post(import::import_data))
        .route("/api/ciphers/{id}", get(ciphers::get_cipher))
        .route("/api/ciphers/{id}", put(ciphers::update_cipher))
        .route("/api/ciphers/{id}", post(ciphers::update_cipher))  // POST 方法更新 (Bitwarden 兼容)
        // Cipher soft delete (PUT sets deleted_at timestamp)
        .route("/api/ciphers/{id}/delete", put(ciphers::soft_delete_cipher))
        .route("/api/ciphers/{id}/delete", post(ciphers::soft_delete_cipher))
        // Cipher hard delete (DELETE permanently removes cipher)
        .route("/api/ciphers/{id}", delete(ciphers::hard_delete_cipher))
        // Cipher restore (clears deleted_at)
        .route("/api/ciphers/{id}/restore", put(ciphers::restore_cipher))
        // Partial update for folder/favorite
        .route("/api/ciphers/{id}/partial", put(ciphers::update_cipher_partial))
        .route("/api/ciphers/{id}/partial", post(ciphers::update_cipher_partial))
        // Folders CRUD
        .route("/api/folders", post(folders::create_folder))
        .route("/api/folders/{id}", put(folders::update_folder))
        .route("/api/folders/{id}", delete(folders::delete_folder))
        .route("/api/config", get(config::config))
        // Emergency access (stub - returns empty lists, feature not supported)
        .route("/api/emergency-access/trusted", get(emergency_access::get_trusted_contacts))
        .route("/api/emergency-access/granted", get(emergency_access::get_granted_access))
        // Devices (stub - device tracking not implemented, JWT-based auth)
        .route("/api/devices", get(devices::get_devices))
        .route("/api/devices/knowndevice", get(devices::get_known_device))
        .route("/api/devices/identifier/{device_id}", get(devices::get_device))
        .route("/api/devices/identifier/{device_id}/token", post(devices::post_device_token))
        .route("/api/devices/identifier/{device_id}/token", put(devices::put_device_token))
        .route("/api/devices/identifier/{device_id}/clear-token", put(devices::put_clear_device_token))
        .route("/api/devices/identifier/{device_id}/clear-token", post(devices::post_clear_device_token))
        // WebAuthn (stub - prevents 404 errors, passkeys not supported)
        .route("/api/webauthn", get(webauth::get_webauthn_credentials))
        .with_state(app_state)
}

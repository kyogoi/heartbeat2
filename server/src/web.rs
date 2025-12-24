use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

use hyper::StatusCode;

use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::auth;
use crate::auth::Password;
use crate::db::Database;

#[derive(Deserialize)]
struct RegistrationRequest {
    token: String,
}

#[derive(Serialize)]
struct RegistrationResponse {
    username: String,
    password: String,
    mqtt_host: String,
    mqtt_port: u16,
}

// route serves token

// pub fn create_router() -> Result<Json<TokenResponse>> {}

// route takes token and serves username password after accepting
// should only ever fire once

pub async fn register_with_token(
    State(db): State<Arc<Database>>,
    Json(payload): Json<RegistrationRequest>,
) -> Result<Json<RegistrationResponse>, (StatusCode, String)> {
    let token = payload.token;

    let device_id = match db.get_device_id_from_token(&token) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid or expired token".to_string(),
            ));
        }
    };

    let username = auth::generate_username();
    let password: Password = Password::new().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate password".to_string(),
        )
    })?;

    // change these later
    let mqtt_host = "localhost";
    let mqtt_port = 1883;

    db.create_device_settings(&device_id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Registration Failed".to_string(),
        )
    })?;

    db.create_device_credentials(&device_id, &username, &password)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Registration Failed".to_string(),
            )
        })?;

    db.create_device_status(&device_id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Registration Failed".to_string(),
        )
    })?;

    db.mark_token_as_used(&token).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Registration Failed".to_string(),
        )
    })?;

    Ok(Json(RegistrationResponse {
        username,
        password: password.plain,
        mqtt_host: mqtt_host.to_string(),
        mqtt_port,
    }))
}

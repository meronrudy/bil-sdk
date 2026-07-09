use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use control_plane::{ControlPlane, ControlPlaneConfig};
use risk_layer::{RiskLayer, RiskLayerInput};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    control_plane: Arc<ControlPlane>,
    api_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_token = env::var("AISSURANCE_API_TOKEN").unwrap_or_else(|_| "dev-token".to_string());

    let config = ControlPlaneConfig::default();
    let risk_layer = RiskLayer::default();
    let control_plane = Arc::new(ControlPlane::new(config, risk_layer));

    let state = AppState {
        control_plane,
        api_token,
    };

    let app = Router::new()
        .route("/api/v1/jobs", post(submit_job))
        .route("/api/v1/jobs/:job_id", get(get_job_status))
        .route("/api/v1/jobs/:job_id/report", get(get_job_report))
        .with_state(state);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn check_auth(headers: &HeaderMap, expected_token: &str) -> Result<(), StatusCode> {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if token == expected_token {
                    return Ok(());
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

async fn submit_job(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<RiskLayerInput>,
) -> Result<impl IntoResponse, StatusCode> {
    check_auth(&headers, &state.api_token)?;

    match state.control_plane.submit_batch(input) {
        Ok(record) => Ok((StatusCode::CREATED, Json(record))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_job_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    check_auth(&headers, &state.api_token)?;

    match state.control_plane.job_status(&job_id) {
        Ok(record) => Ok((StatusCode::OK, Json(record))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_job_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    check_auth(&headers, &state.api_token)?;

    match state.control_plane.report(&job_id) {
        Ok(report) => Ok((StatusCode::OK, Json(report))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

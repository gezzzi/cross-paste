use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;

use crate::api_types::{ApiResponse, ClipboardContent, HealthResponse};
use crate::clipboard;
use crate::settings::AppState;

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let settings = state.settings.lock().await;
    let expected_key = settings.api_key.clone();
    drop(settings);

    let authorized = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|token| token == expected_key)
        .unwrap_or(false);

    if !authorized {
        let resp = ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Invalid API key".to_string()),
        };
        return (StatusCode::UNAUTHORIZED, Json(resp)).into_response();
    }

    next.run(request).await
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn get_clipboard_handler() -> Result<Json<ApiResponse<ClipboardContent>>, StatusCode> {
    let result = tokio::task::spawn_blocking(clipboard::read_clipboard_text)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Ok(text) => Ok(Json(ApiResponse {
            success: true,
            data: Some(ClipboardContent {
                content_type: "text".to_string(),
                data: text,
            }),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

async fn post_clipboard_handler(
    Json(payload): Json<ClipboardContent>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let text = payload.data;
    let result = tokio::task::spawn_blocking(move || clipboard::write_clipboard_text(&text))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Ok(()) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

pub async fn start_server(state: Arc<AppState>) -> Result<oneshot::Sender<()>, String> {
    let settings = state.settings.lock().await;
    let port = settings.port;
    let bind_address = settings.bind_address.clone();
    drop(settings);

    let auth_routes = Router::new()
        .route("/api/clipboard", get(get_clipboard_handler))
        .route("/api/clipboard", post(post_clipboard_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .merge(auth_routes)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr: SocketAddr = format!("{bind_address}:{port}")
        .parse()
        .map_err(|e| format!("Invalid bind address: {e}"))?;

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("Failed to bind to {addr}: {e}"))?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
            })
            .await
            .ok();
    });

    Ok(shutdown_tx)
}

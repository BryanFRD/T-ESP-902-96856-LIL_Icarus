use axum::{
    body::{Body, to_bytes},
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use http::Method;
use serde_json::{Value as JsonValue, json};
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::time::Instant;
use tower_cookies::Cookies;
use tracing::error;

use crate::api::auth::models::JwtClaims;
use crate::api::events::{
    EventAction, EventChanges, NewEvent, ResourceFetcherRegistry, ResourceType, log_event_sync,
};
use crate::state::GlobalState;
use crate::utils::event_encryption::encrypt_json;
use crate::utils::redaction::redact_json_value;

use crate::api::events::logging::{
    RoutePattern, infer_delete_event, infer_get_event, infer_patch_event, infer_post_event,
};

#[derive(Clone)]
pub struct EventLoggingConfig {
    pub track_reads: bool,
    pub excluded_paths: Vec<String>,
}

impl Default for EventLoggingConfig {
    fn default() -> Self {
        Self {
            track_reads: true,
            excluded_paths: vec![
                "/v1/status".to_string(),
                "/v1/swagger".to_string(),
                "/v1/events".to_string(),
            ],
        }
    }
}

#[allow(clippy::too_many_lines)]
pub async fn event_logging_middleware(
    State(state): State<GlobalState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();

    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let query = req.uri().query().map(String::from);

    let user_id = extract_user_id_from_cookies(&cookies);

    let ip_hash = Some(anonymize_ip_address(&addr.ip().to_string()));
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(String::from);

    let referer = req
        .headers()
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .map(String::from);

    let config = EventLoggingConfig::default();
    let should_log = should_log_request(&method, &path, &config);

    let event_details = if should_log {
        infer_event_details(&method, &path)
    } else {
        None
    };

    let before_state = if let Some((ref action, ref resource_type, ref resource_id)) = event_details
    {
        if matches!(action, EventAction::Update | EventAction::Delete) {
            ResourceFetcherRegistry::fetch_before_state(&state, resource_type, resource_id.as_ref())
        } else {
            None
        }
    } else {
        None
    };

    let (req, request_body_json) =
        if should_log && matches!(method, Method::PATCH | Method::POST | Method::PUT) {
            match buffer_and_clone_body(req).await {
                Ok((new_req, body_json)) => (new_req, body_json),
                Err(original_req) => (original_req, None),
            }
        } else {
            (req, None)
        };

    let response = next.run(req).await;

    let duration_ms = start_time.elapsed().as_millis();
    let status_code = response.status().as_u16();

    if should_log && let Some((action, resource_type, resource_id)) = event_details {
        let mut metadata = json!({
            "response_status": status_code,
            "duration_ms": duration_ms,
        });

        if let Some(q) = query {
            metadata["query_params"] = json!(q);
        }

        if let Some(ref_url) = referer {
            metadata["referer"] = json!(ref_url);
        }

        if !response.status().is_success() {
            metadata["is_error"] = json!(true);
        }

        let changes = build_event_changes(&action, before_state, request_body_json);

        let mut new_event = NewEvent::new(
            user_id,
            action,
            resource_type,
            resource_id,
            method.to_string(),
            path,
        )
        .with_ip_address(ip_hash)
        .with_user_agent(user_agent);

        if !metadata.is_null() && metadata.as_object().is_some_and(|o| !o.is_empty()) {
            match encrypt_json(&state.event_encryption_key, &metadata) {
                Ok((ciphertext, nonce)) => {
                    new_event = new_event
                        .with_metadata(serde_json::json!(ciphertext))
                        .with_metadata_nonce(Some(nonce));
                }
                Err(e) => {
                    error!("Failed to encrypt metadata: {}", e);
                    new_event = new_event.with_metadata(metadata);
                }
            }
        }

        if let Some(ref changes) = changes
            && let Ok(changes_json) = serde_json::to_value(changes)
        {
            match encrypt_json(&state.event_encryption_key, &changes_json) {
                Ok((ciphertext, nonce)) => {
                    new_event = new_event
                        .with_changes(EventChanges {
                            before: None,
                            after: Some(serde_json::json!(ciphertext)),
                        })
                        .with_changes_nonce(Some(nonce));
                }
                Err(_) => {
                    new_event = new_event.with_changes(changes.clone());
                }
            }
        }

        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = log_event_sync(&state_clone, &new_event) {
                error!("Failed to log event: {}", e);
            }
        });
    }

    response
}

fn build_event_changes(
    action: &EventAction,
    before_state: Option<JsonValue>,
    request_body: Option<JsonValue>,
) -> Option<EventChanges> {
    match action {
        EventAction::Update => {
            let before = before_state.map(|mut v| {
                redact_json_value(&mut v, true);
                v
            });
            let after = request_body.map(|mut v| {
                redact_json_value(&mut v, true);
                v
            });

            if before.is_some() || after.is_some() {
                Some(EventChanges::new(before, after))
            } else {
                None
            }
        }
        EventAction::Delete => before_state.map(|mut v| {
            redact_json_value(&mut v, true);
            EventChanges::new(Some(v), None)
        }),
        EventAction::Create => request_body.map(|mut v| {
            redact_json_value(&mut v, true);
            EventChanges::new(None, Some(v))
        }),
        _ => None,
    }
}

fn extract_user_id_from_cookies(cookies: &Cookies) -> Option<String> {
    let auth_cookie = cookies.get("auth")?;
    let jwt_secret = std::env::var("JWT_SECRET").ok()?;
    let jwt = JwtClaims::from_jwt(auth_cookie.value(), &jwt_secret);

    if jwt.is_valid() {
        Some(jwt.sub.to_string())
    } else {
        None
    }
}

fn should_log_request(method: &Method, path: &str, config: &EventLoggingConfig) -> bool {
    if config.excluded_paths.iter().any(|p| path.starts_with(p)) {
        return false;
    }

    if !config.track_reads && method == Method::GET {
        return false;
    }

    true
}

fn anonymize_ip_address(ip: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn infer_event_details(
    method: &Method,
    path: &str,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    let route = RoutePattern::new(method.as_str(), path);

    match route.method {
        "POST" => infer_post_event(&route),
        "GET" => infer_get_event(&route),
        "PATCH" => infer_patch_event(&route),
        "DELETE" => infer_delete_event(&route),
        _ => None,
    }
}

async fn buffer_and_clone_body(req: Request) -> Result<(Request, Option<JsonValue>), Request> {
    let (parts, body) = req.into_parts();

    let Ok(bytes) = to_bytes(body, 1024 * 1024).await else {
        let req = Request::from_parts(parts, Body::empty());
        return Err(req);
    };

    let body_json = serde_json::from_slice::<JsonValue>(&bytes).ok();

    let new_body = Body::from(bytes);
    let req = Request::from_parts(parts, new_body);

    Ok((req, body_json))
}

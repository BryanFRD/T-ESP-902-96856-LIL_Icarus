use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

/// Middleware to track HTTP request metrics for Prometheus
///
/// Tracks:
/// - Request duration (histogram)
/// - Request count (counter)
/// - Requests in flight (gauge)
pub async fn metrics_middleware(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path = normalize_path(req.uri().path());

    // Increment in-flight requests gauge
    metrics::gauge!("http_requests_in_flight", "method" => method.clone(), "path" => path.clone())
        .increment(1.0);

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed().as_secs_f64();

    // Decrement in-flight requests gauge
    metrics::gauge!("http_requests_in_flight", "method" => method.clone(), "path" => path.clone())
        .decrement(1.0);

    let status_code = response.status().as_u16();
    let status = status_code.to_string();

    // Record request duration
    metrics::histogram!(
        "http_request_duration_seconds",
        "method" => method.clone(),
        "path" => path.clone(),
        "status" => status.clone()
    )
    .record(duration);

    // Increment request counter
    metrics::counter!(
        "http_requests_total",
        "method" => method.clone(),
        "path" => path.clone(),
        "status" => status.clone()
    )
    .increment(1);

    // Track errors (4xx/5xx)
    if status_code >= 400 {
        let error_type = if status_code >= 500 {
            "server_error"
        } else {
            "client_error"
        };
        metrics::counter!(
            "http_errors_total",
            "method" => method,
            "path" => path,
            "status" => status,
            "error_type" => error_type
        )
        .increment(1);
    }

    response
}

/// Normalize path to group dynamic segments
///
/// Examples:
/// - `/v1/teams/123` -> `/v1/teams/:id`
/// - `/v1/teams/456/invitations` -> `/v1/teams/:id/invitations`
/// - `/v1/users/789` -> `/v1/users/:id`
/// - `/v1/audits/abc-def` -> `/v1/audits/:id`
fn normalize_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    let mut normalized = Vec::new();

    for (i, segment) in segments.iter().enumerate() {
        if segment.is_empty() {
            continue;
        }

        // Check if segment looks like an ID (UUID, number, or alphanumeric)
        // Skip normalization for known static segments
        let is_likely_id = match i {
            // After /v1/teams, /v1/users, /v1/audits, etc., next segment is likely an ID
            i if i > 0 && matches!(
                segments.get(i - 1),
                Some(&"teams" | &"users" | &"audits" | &"events")
            ) => {
                // If followed by a known sub-resource, it's an ID
                segment.chars().any(|c| c.is_ascii_digit() || c == '-')
            }
            _ => false,
        };

        if is_likely_id {
            normalized.push(":id");
        } else {
            normalized.push(segment);
        }
    }

    format!("/{}", normalized.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/v1/status"), "/v1/status");
        assert_eq!(normalize_path("/v1/teams/123"), "/v1/teams/:id");
        assert_eq!(
            normalize_path("/v1/teams/456/invitations"),
            "/v1/teams/:id/invitations"
        );
        assert_eq!(normalize_path("/v1/users/789"), "/v1/users/:id");
        assert_eq!(
            normalize_path("/v1/audits/abc-123-def"),
            "/v1/audits/:id"
        );
        assert_eq!(normalize_path("/metrics"), "/metrics");
        assert_eq!(normalize_path("/v1/auth/login"), "/v1/auth/login");
    }
}

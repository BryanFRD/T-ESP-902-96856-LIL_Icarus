use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::info;

use crate::state::GlobalState;

pub fn cleanup_expired_events(state: &GlobalState) -> Result<usize, diesel::result::Error> {
    use crate::schema::events::dsl::{events, expires_at};

    let mut conn = state.get_db_conn().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    let now = Utc::now().naive_utc();
    let deleted = diesel::delete(events.filter(expires_at.lt(now))).execute(&mut conn)?;

    if deleted > 0 {
        info!("Cleaned up {} expired events", deleted);
    }

    Ok(deleted)
}

pub async fn start_cleanup_task(state: GlobalState) {
    use tokio::time::{Duration, interval};

    let mut cleanup_interval = interval(Duration::from_secs(3600));

    loop {
        cleanup_interval.tick().await;

        match cleanup_expired_events(&state) {
            Ok(count) => {
                if count > 0 {
                    info!("Automatic cleanup removed {} expired events", count);
                }
            }
            Err(e) => {
                tracing::error!("Failed to cleanup expired events: {}", e);
            }
        }
    }
}

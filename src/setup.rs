use std::{mem::swap, sync::Arc};

use anyhow::anyhow;
use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use tokio::sync::RwLock;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultOnFailure, DefaultOnResponse, TraceLayer},
};
use tracing::{info_span, Level};
use uuid::Uuid;

use crate::{
    constants::{GRUE_PATH, HEALTH_PATH, RESET_PATH, VEHICLE_PATH},
    latest_grue_data::LatestGrueData,
    routes,
};

pub struct AppState {
    pub latest_grue_data: RwLock<LatestGrueData>,
    pub lock_uuid: Option<RwLock<Uuid>>,
}

impl AppState {
    pub async fn reset_uuid(&self, uuid: Uuid) -> anyhow::Result<()> {
        match &self.lock_uuid {
            Some(lock_uuid) => {
                if uuid == *lock_uuid.read().await {
                    let mut lock = self.latest_grue_data.write().await;

                    swap(&mut *lock, &mut LatestGrueData::default());

                    Ok(())
                } else {
                    Err(anyhow!("Error: The UUID supplied does not match the UUID supplied at server start."))
                }
            }
            None => Err(anyhow!(
                "Error: No reset allowed. No UUID supplied when the server started."
            )),
        }
    }
}

/// Setup the Axum Server with routing
pub fn generate_router(maybe_uuid: Option<Uuid>) -> Router {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    add_http_tracing(
        Router::new()
            .route(GRUE_PATH, post(routes::post_grue_data))
            .route(VEHICLE_PATH, get(routes::get_vehicle_data))
            .route(HEALTH_PATH, get(routes::get_health))
            .route(RESET_PATH, post(routes::reset))
            .layer(cors)
            .with_state(Arc::new(AppState {
                latest_grue_data: Default::default(),
                lock_uuid: maybe_uuid.map(RwLock::new),
            })),
    )
}

/// Wrap a tracing layer around a router to trace HTTP calls
fn add_http_tracing(router: Router) -> Router {
    router.layer(
        // https://docs.rs/tower-http/latest/tower_http/trace/index.html#when-the-callbacks-are-called
        TraceLayer::new_for_http()
            .make_span_with(|req: &Request<_>| {
                let method = req.method().as_str();
                let path = req
                    .extensions()
                    .get::<MatchedPath>()
                    .map_or_else(|| req.uri().path(), |path| path.as_str());

                info_span!("http_req", %method, %path)
            })
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
    )
}

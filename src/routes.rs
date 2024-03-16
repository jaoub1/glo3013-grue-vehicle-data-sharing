use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app_state::AppState, latest_grue_data::LatestGrueData, loading_zone::LoadingZone};

#[derive(Deserialize, Serialize)]
pub struct GrueRequest {
    pub grue_id: u8,
    pub number_of_merchandise: u8,
}

#[derive(Serialize, Debug)]
pub struct VehicleResponse {
    #[allow(dead_code)]
    vehicle_data: LatestGrueData,
}

#[derive(Deserialize, Serialize)]
pub struct ResetRequest {
    #[allow(dead_code)]
    uuid: Uuid,
}

pub async fn post_grue_data(
    State(app): State<Arc<AppState>>,
    request: Json<GrueRequest>,
) -> impl IntoResponse {
    match LoadingZone::try_from(request.grue_id) {
        Ok(zone_id) => {
            app.latest_grue_data
                .write()
                .await
                .update_data(zone_id, request.number_of_merchandise);
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_health() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn get_vehicle_data(State(app): State<Arc<AppState>>) -> Json<VehicleResponse> {
    Json(VehicleResponse {
        vehicle_data: app.latest_grue_data.read().await.clone(),
    })
}

pub async fn reset(
    State(app): State<Arc<AppState>>,
    maybe_request: Option<Json<ResetRequest>>,
) -> impl IntoResponse {
    match app
        .reset_uuid(maybe_request.map(|request| request.uuid))
        .await
    {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn version() -> impl IntoResponse {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use axum_test::{TestServer, TestServerConfig};
    use serde_json::json;
    use uuid::uuid;

    use crate::{
        constants::{GRUE_PATH, HEALTH_PATH, RESET_PATH, VEHICLE_PATH},
        setup,
    };

    use super::*;

    const VALID_GRUE_ID: u8 = 1;
    const INVALID_GRUE_ID: u8 = LoadingZone::MAX_ID + 1;
    const ANY_NUMBER_OF_MERCHANDISE: u8 = 3;
    const VALID_UUID: Uuid = uuid!("bb3b9185-f6a8-49eb-b5de-9fb50ff441e4");
    const INVALID_UUID: Uuid = uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff");

    fn given_test_server(maybe_uuid: Option<Uuid>) -> TestServer {
        TestServer::new_with_config(
            setup::generate_router(maybe_uuid),
            TestServerConfig::builder()
                .expect_success_by_default()
                .save_cookies()
                .build(),
        )
        .expect("Cannot create TestServer with a router")
    }

    #[tokio::test]
    async fn given_valid_grue_request_when_post_grue_data_then_ok() {
        let server = given_test_server(None);
        let body = json!({
            "grue_id": VALID_GRUE_ID,
            "number_of_merchandise": ANY_NUMBER_OF_MERCHANDISE
        });

        let response = server.post(GRUE_PATH).json(&body).await;

        response.assert_status(StatusCode::OK);
    }

    #[tokio::test]
    async fn given_app_running_when_get_health_then_ok() {
        let server = given_test_server(None);
        let response = server.get(HEALTH_PATH).await;

        response.assert_status(StatusCode::OK);
    }

    #[tokio::test]
    async fn given_invalid_grue_request_when_post_grue_data_then_bad_request() {
        let server = given_test_server(None);
        let body = json!({
            "grue_id": INVALID_GRUE_ID,
            "number_of_merchandise": ANY_NUMBER_OF_MERCHANDISE
        });

        let response = server.post(GRUE_PATH).json(&body).expect_failure().await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn given_some_grue_data_when_get_vehicle_data_then_ok_with_data() {
        let server = given_test_server(None);
        let body = GrueRequest {
            grue_id: VALID_GRUE_ID,
            number_of_merchandise: ANY_NUMBER_OF_MERCHANDISE,
        };

        let _ = server.post(GRUE_PATH).json(&body).await;
        let response = server.get(VEHICLE_PATH).json(&body).await;

        response.assert_status(StatusCode::OK);
        response.assert_json(&json!({
            "vehicle_data" : {
                "zone1": ANY_NUMBER_OF_MERCHANDISE,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
    }

    #[tokio::test]
    async fn given_new_grue_data_when_get_vehicle_data_then_ok_with_new_data() {
        let server = given_test_server(None);
        let body = GrueRequest {
            grue_id: 42,
            number_of_merchandise: ANY_NUMBER_OF_MERCHANDISE,
        };

        let response1 = server.get(VEHICLE_PATH).json(&body).await;
        let _ = server.post(GRUE_PATH).json(&body).await;
        let response2 = server.get(VEHICLE_PATH).json(&body).await;

        response1.assert_json(&json!({
            "vehicle_data" : {
                "zone1": 0,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0,
            }
        }));
        response2.assert_json(&json!({
            "vehicle_data" : {
                "zone1": 0,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0,
                "zone42": ANY_NUMBER_OF_MERCHANDISE,
            }
        }));
    }

    #[tokio::test]
    async fn given_valid_uuid_when_post_reset_then_ok_with_default() {
        let server = given_test_server(Some(VALID_UUID));
        let body_grue = GrueRequest {
            grue_id: VALID_GRUE_ID,
            number_of_merchandise: ANY_NUMBER_OF_MERCHANDISE,
        };
        let body_reset = ResetRequest { uuid: VALID_UUID };

        let _ = server.post(GRUE_PATH).json(&body_grue).await;
        let response1 = server.get(VEHICLE_PATH).await;
        let _ = server.post(RESET_PATH).json(&body_reset).await;
        let response2 = server.get(VEHICLE_PATH).await;

        response1.assert_json(&json!({
            "vehicle_data" : {
                "zone1": ANY_NUMBER_OF_MERCHANDISE,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
        response2.assert_json(&json!({
            "vehicle_data" : {
                "zone1": 0,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
    }

    #[tokio::test]
    async fn given_invalid_uuid_when_post_reset_then_err() {
        let server = given_test_server(Some(VALID_UUID));
        let body_grue = GrueRequest {
            grue_id: VALID_GRUE_ID,
            number_of_merchandise: ANY_NUMBER_OF_MERCHANDISE,
        };
        let body_reset = ResetRequest { uuid: INVALID_UUID };

        let _ = server.post(GRUE_PATH).json(&body_grue).await;
        let response1 = server.get(VEHICLE_PATH).await;
        let response2 = server
            .post(RESET_PATH)
            .json(&body_reset)
            .expect_failure()
            .await;
        let response3 = server.get(VEHICLE_PATH).await;

        response2.assert_status(StatusCode::BAD_REQUEST);
        response1.assert_json(&json!({
            "vehicle_data" : {
                "zone1": ANY_NUMBER_OF_MERCHANDISE,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
        response3.assert_json(&json!({
            "vehicle_data" : {
                "zone1": ANY_NUMBER_OF_MERCHANDISE,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
    }

    #[tokio::test]
    async fn given_no_initial_uuid_when_post_reset_then_ok_with_default() {
        let server = given_test_server(None);
        let body_grue = GrueRequest {
            grue_id: VALID_GRUE_ID,
            number_of_merchandise: ANY_NUMBER_OF_MERCHANDISE,
        };

        let _ = server.post(GRUE_PATH).json(&body_grue).await;
        let response1 = server.get(VEHICLE_PATH).await;
        let _ = server.post(RESET_PATH).await;
        let response3 = server.get(VEHICLE_PATH).await;

        response1.assert_json(&json!({
            "vehicle_data" : {
                "zone1": ANY_NUMBER_OF_MERCHANDISE,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
        response3.assert_json(&json!({
            "vehicle_data" : {
                "zone1": 0,
                "zone2": 0,
                "zone3": 0,
                "zone4": 0,
                "zone5": 0,
                "zone6": 0
            }
        }));
    }
}

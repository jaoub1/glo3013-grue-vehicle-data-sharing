use std::{fmt::Display, sync::Arc};

use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize, Serializer};

use crate::{latest_grue_data::LatestGrueData, setup::AppState};

#[derive(Deserialize, Serialize)]
pub struct GrueRequest {
    pub grue_id: u8,
    pub number_of_merchandise: u8,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TeamId(u8);

impl TeamId {
    pub const MIN_ID: u8 = 0;
    pub const MAX_ID: u8 = 14;
    pub const DEFAULT_NUMBER_OF_GRUE: usize = 6;
}

impl Serialize for TeamId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("team{}", self.0))
    }
}

impl Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "team{}", self.0)
    }
}

impl TryFrom<u8> for TeamId {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (Self::MIN_ID..=Self::MAX_ID).contains(&value) {
            Ok(Self(value))
        } else {
            Err(anyhow!(
                "Error: A TeamId 'team_id' field must be between {} and {}",
                Self::MIN_ID,
                Self::MAX_ID
            ))
        }
    }
}

#[derive(Serialize, Debug)]
pub struct VehicleResponse {
    #[allow(dead_code)]
    vehicle_data: LatestGrueData,
}

pub async fn post_grue_data(
    State(app): State<Arc<AppState>>,
    request: Json<GrueRequest>,
) -> impl IntoResponse {
    match TeamId::try_from(request.grue_id) {
        Ok(team_id) => {
            app.latest_grue_data
                .write()
                .await
                .update_data(team_id, request.number_of_merchandise);
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_vehicle_data(State(app): State<Arc<AppState>>) -> Json<VehicleResponse> {
    Json(VehicleResponse {
        vehicle_data: app.latest_grue_data.read().await.clone(),
    })
}

#[cfg(test)]
mod tests {
    use axum_test::{TestServer, TestServerConfig};
    use serde_json::json;

    use crate::{
        constants::{GRUE_PATH, VEHICLE_PATH},
        setup,
    };

    use super::*;

    const VALID_GRUE_ID: u8 = 1;
    const INVALID_GRUE_ID: u8 = 42;
    const VALID_NUMBER_OF_MERCHANDISE: u8 = 3;

    fn given_test_server() -> TestServer {
        TestServer::new_with_config(
            setup::generate_router(),
            TestServerConfig::builder()
                .expect_success_by_default()
                .save_cookies()
                .build(),
        )
        .expect("Cannot create TestServer with a router")
    }

    #[tokio::test]
    async fn given_valid_grue_request_when_post_grue_data_then_ok() {
        let server = given_test_server();
        let body = json!({
            "grue_id": VALID_GRUE_ID,
            "number_of_merchandise": VALID_NUMBER_OF_MERCHANDISE
        });

        let response = server.post(GRUE_PATH).json(&body).await;

        response.assert_status(StatusCode::OK);
    }

    #[tokio::test]
    async fn given_invalid_grue_request_when_post_grue_data_then_bad_request() {
        let server = given_test_server();
        let body = json!({
            "grue_id": INVALID_GRUE_ID,
            "number_of_merchandise": VALID_NUMBER_OF_MERCHANDISE
        });

        let response = server.post(GRUE_PATH).json(&body).expect_failure().await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn given_some_grue_data_when_get_vehicle_data_then_ok_with_data() {
        let server = given_test_server();
        let body = GrueRequest {
            grue_id: VALID_GRUE_ID,
            number_of_merchandise: VALID_NUMBER_OF_MERCHANDISE,
        };

        let _ = server.post(GRUE_PATH).json(&body).await;
        let response = server.get(VEHICLE_PATH).json(&body).await;

        response.assert_status(StatusCode::OK);
        response.assert_json(&json!({
            "vehicle_data" : {
                "team1": 3,
                "team2": 0,
                "team3": 0,
                "team4": 0,
                "team5": 0,
                "team6": 0
            }
        }));
    }
}

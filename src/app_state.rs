use std::mem::swap;

use anyhow::anyhow;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::latest_grue_data::LatestGrueData;

pub struct AppState {
    pub latest_grue_data: RwLock<LatestGrueData>,
    pub lock_uuid: Option<RwLock<Uuid>>,
}

impl AppState {
    pub async fn reset_uuid(&self, uuid: Option<Uuid>) -> anyhow::Result<()> {
        if let Some(lock_uuid) = &self.lock_uuid {
            if uuid != Some(*lock_uuid.read().await) {
                return Err(anyhow!(
                    "Error: The UUID supplied does not match the UUID supplied at server start."
                ));
            }
        }

        let mut lock = self.latest_grue_data.write().await;
        swap(&mut *lock, &mut LatestGrueData::default());

        Ok(())
    }
}

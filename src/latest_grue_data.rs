use std::collections::HashMap;

use serde::Serialize;

use crate::loading_zone::LoadingZone;

#[derive(Clone, Serialize, Debug)]
pub struct LatestGrueData(HashMap<LoadingZone, u8>);

impl LatestGrueData {
    pub fn update_data(&mut self, zone_id: LoadingZone, number_of_merchandise: u8) {
        self.0
            .entry(zone_id)
            .and_modify(|x| *x = number_of_merchandise)
            .or_insert(number_of_merchandise);
    }

    pub fn get_marchandise(&self, zone: LoadingZone) -> Option<&u8> {
        self.0.get(&zone)
    }
}

impl Default for LatestGrueData {
    fn default() -> Self {
        Self(
            LoadingZone::DEFAULT_LOADING_ZONES
                .map(LoadingZone::try_from)
                .map(|r| r.expect("Default LatestGrueData must be valid"))
                .map(|id| (id, 0))
                .collect(),
        )
    }
}

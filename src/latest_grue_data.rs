use std::collections::HashMap;

use serde::Serialize;

use crate::routes::TeamId;

#[derive(Clone, Serialize, Debug)]
pub struct LatestGrueData(HashMap<TeamId, u8>);

impl LatestGrueData {
    pub fn update_data(&mut self, team_id: TeamId, number_of_merchandise: u8) {
        self.0
            .entry(team_id)
            .and_modify(|x| *x = number_of_merchandise)
            .or_insert(number_of_merchandise);
    }
}

impl Default for LatestGrueData {
    fn default() -> Self {
        Self(
            (1..=TeamId::DEFAULT_NUMBER_OF_GRUE)
                .map(|id| TeamId::try_from(id as u8))
                .map(|r| r.expect("Default LatestGrueData must be valid"))
                .map(|id| (id, 0))
                .collect(),
        )
    }
}

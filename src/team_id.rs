use std::fmt::Display;

use anyhow::anyhow;
use serde::{Serialize, Serializer};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TeamId(u8);

impl TeamId {
    pub const MIN_ID: u8 = 1;
    pub const MAX_ID: u8 = 6;
    pub const DEFAULT_NUMBER_OF_GRUE: usize = Self::MAX_ID as usize;
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

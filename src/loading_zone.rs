use std::fmt::Display;

use anyhow::anyhow;
use serde::{Serialize, Serializer};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LoadingZone(pub u8);

impl LoadingZone {
    pub const MIN_ID: u8 = 0;
    pub const MAX_ID: u8 = 99;
    pub const DEFAULT_LOADING_ZONES: std::ops::RangeInclusive<u8> = 1..=6;
}

impl Serialize for LoadingZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for LoadingZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zone{}", self.0)
    }
}

impl TryFrom<u8> for LoadingZone {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (LoadingZone::MIN_ID..=LoadingZone::MAX_ID).contains(&value) {
            Ok(Self(value))
        } else {
            Err(anyhow!(
                "Error: A LoadingZone must be between {} and {}",
                Self::MIN_ID,
                Self::MAX_ID
            ))
        }
    }
}

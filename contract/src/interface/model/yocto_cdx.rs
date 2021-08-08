use crate::domain;
use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct YoctoCdx(pub U128);

impl From<domain::YoctoCdx> for YoctoCdx {
    fn from(value: domain::YoctoCdx) -> Self {
        Self(value.0.into())
    }
}

impl From<u128> for YoctoCdx {
    fn from(value: u128) -> Self {
        Self(value.into())
    }
}

impl YoctoCdx {
    pub fn value(&self) -> u128 {
        self.0 .0
    }
}

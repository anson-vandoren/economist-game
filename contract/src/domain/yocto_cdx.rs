use crate::core::U256;
use crate::interface;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default,
)]
pub struct YoctoCdx(pub u128);

impl From<u128> for YoctoCdx {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl YoctoCdx {
    pub fn value(&self) -> u128 {
        self.0
    }
}

impl From<YoctoCdx> for u128 {
    fn from(value: YoctoCdx) -> Self {
        value.0
    }
}

impl From<YoctoCdx> for U128 {
    fn from(value: YoctoCdx) -> Self {
        value.0.into()
    }
}

impl From<U128> for YoctoCdx {
    fn from(value: U128) -> Self {
        Self(value.0)
    }
}

impl From<interface::YoctoCdx> for YoctoCdx {
    fn from(value: interface::YoctoCdx) -> Self {
        Self(value.0 .0)
    }
}

impl Deref for YoctoCdx {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YoctoCdx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for YoctoCdx {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<YoctoCdx> for U256 {
    fn from(value: YoctoCdx) -> Self {
        U256::from(value.value())
    }
}

impl Sub for YoctoCdx {
    type Output = YoctoCdx;

    fn sub(self, rhs: Self) -> Self::Output {
        YoctoCdx(
            self.0
                .checked_sub(rhs.0)
                .expect("attempt to subtract with overflow"),
        )
    }
}

impl SubAssign for YoctoCdx {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self
            .0
            .checked_sub(rhs.0)
            .expect("attempt to subtract with overflow")
    }
}

impl Add for YoctoCdx {
    type Output = YoctoCdx;

    fn add(self, rhs: Self) -> Self::Output {
        YoctoCdx(
            self.0
                .checked_add(rhs.0)
                .expect("attempt to add with overflow"),
        )
    }
}

impl AddAssign for YoctoCdx {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self
            .0
            .checked_add(rhs.0)
            .expect("attempt to add with overflow")
    }
}

use crate::domain::{BlockHeight, BlockTimestamp, EpochHeight, YoctoCdx};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
};
use std::cmp::Ordering;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy)]
pub struct TimestampedCdxBalance {
    amount: YoctoCdx,
    block_height: BlockHeight,
    block_timestamp: BlockTimestamp,
    epoch_height: EpochHeight,
}

impl PartialEq for TimestampedCdxBalance {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl PartialEq<u128> for TimestampedCdxBalance {
    fn eq(&self, other: &u128) -> bool {
        self.amount.0 == *other
    }
}

impl PartialOrd for TimestampedCdxBalance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.amount.cmp(&other.amount))
    }
}

impl PartialOrd<u128> for TimestampedCdxBalance {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        Some(self.amount.0.cmp(other))
    }
}

impl TimestampedCdxBalance {
    /// ## Panics
    /// if the NEAR runtime context is not available
    pub fn new(balance: YoctoCdx) -> Self {
        Self {
            amount: balance,
            block_height: env::block_index().into(),
            block_timestamp: env::block_timestamp().into(),
            epoch_height: env::epoch_height().into(),
        }
    }

    pub fn amount(&self) -> YoctoCdx {
        self.amount
    }

    pub fn block_height(&self) -> BlockHeight {
        self.block_height
    }

    pub fn block_timestamp(&self) -> BlockTimestamp {
        self.block_timestamp
    }

    pub fn epoch_height(&self) -> EpochHeight {
        self.epoch_height
    }

    /// ## Panics
    /// if overflow occurs
    pub fn credit(&mut self, amount: YoctoCdx) {
        if amount.0 == 0 {
            return;
        }
        self.amount += amount;
        self.update_timestamp();
    }

    /// ## Panics
    /// if overflow occurs
    pub fn debit(&mut self, amount: YoctoCdx) {
        if amount.0 == 0 {
            return;
        }
        assert!(
            self.amount >= amount,
            "balance is too low to fulfill debit request"
        );
        self.amount -= amount;
        self.update_timestamp();
    }

    fn update_timestamp(&mut self) {
        self.epoch_height = env::epoch_height().into();
        self.block_timestamp = env::block_timestamp().into();
        self.block_height = env::block_index().into();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;
    use near_sdk::{testing_env, MockedBlockchain};

    #[test]
    #[should_panic]
    fn timestamped_balance_new_outside_near_runtime() {
        let _balance = TimestampedCdxBalance::new(10.into());
    }

    #[test]
    fn timestamped_balance_new() {
        let mut context = new_context("bob.near");
        context.block_index = 1;
        context.block_timestamp = 2;
        context.epoch_height = 3;

        testing_env!(context);
        let balance = TimestampedCdxBalance::new(10.into());
        assert_eq!(balance.amount(), 10.into());
        assert_eq!(balance.block_height(), 1.into());
        assert_eq!(balance.block_timestamp(), 2.into());
        assert_eq!(balance.epoch_height(), 3.into());
    }

    #[test]
    pub fn timestamped_balance_partial_eq() {
        let mut context = new_context("bob.near");
        testing_env!(context.clone());

        let balance_1 = TimestampedCdxBalance::new(10.into());

        context.block_index = 10;
        context.block_timestamp = 20;
        context.epoch_height = 30;
        testing_env!(context.clone());
        let balance_2 = TimestampedCdxBalance::new(10.into());

        assert!(balance_1 == balance_2);
        assert!(balance_1 == 10u128);
    }

    #[test]
    pub fn timestamped_balance_debug() {
        let mut context = new_context("bob.near");
        context.block_index = 1;
        context.block_timestamp = 2;
        context.epoch_height = 3;
        testing_env!(context.clone());

        let balance = TimestampedCdxBalance::new(10.into());
        println!("{:?}", balance);
    }

    #[test]
    pub fn timestamped_balance_borsh() {
        let mut context = new_context("bob.near");
        context.block_index = 1;
        context.block_timestamp = 2;
        context.epoch_height = 3;
        testing_env!(context.clone());

        let balance = TimestampedCdxBalance::new(10.into());
        let bytes: Vec<u8> = balance.try_to_vec().unwrap();
        let balance2: TimestampedCdxBalance =
            TimestampedCdxBalance::try_from_slice(&bytes).unwrap();
        assert_eq!(balance, balance2);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    pub fn credit_panics_on_overflow() {
        let context = new_context("bob.near");
        testing_env!(context);

        let mut balance = TimestampedCdxBalance::new(10.into());
        balance.credit(u128::MAX.into());
    }

    #[test]
    #[should_panic(expected = "balance is too low to fulfill debit request")]
    pub fn debit_panics_on_overflow() {
        let context = new_context("bob.near");
        testing_env!(context);

        let mut balance = TimestampedCdxBalance::new(10.into());
        balance.debit(u128::MAX.into());
    }
}

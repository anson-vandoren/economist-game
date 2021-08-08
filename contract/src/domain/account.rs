use crate::core::Hash;
use crate::domain::{TimestampedCdxBalance, TimestampedNearBalance, YoctoCdx, YoctoNear};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::ops::{Deref, DerefMut};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub struct Account {
    /// account holder is responsible to pay their own storage fees
    /// the funds are escrowed and refunded if the account is unregistered
    pub storage_escrow: TimestampedNearBalance,

    /// CDX tokens that the account owns
    pub cdx: Option<TimestampedCdxBalance>,
}

impl Account {
    pub fn new(storage_escrow_fee: YoctoNear) -> Self {
        Self {
            storage_escrow: TimestampedNearBalance::new(storage_escrow_fee),
            cdx: None,
        }
    }

    /// create a fully-populated template [Account] to measure storage that needs to be allocated
    /// which in turn determines how much a user must deposit to open this account
    pub(crate) fn account_template_to_measure_storage_usage() -> Self {
        Self {
            storage_escrow: TimestampedNearBalance::new(0.into()),
            cdx: Some(TimestampedCdxBalance::new(0.into())),
        }
    }

    /// true if there are any tokens (NEAR or otherwise) still credited to this account
    /// used to determine if the account can be safely closed
    pub fn has_funds(&self) -> bool {
        self.cdx.map_or(false, |balance| balance > 0)
    }

    pub fn apply_cdx_credit(&mut self, credit: YoctoCdx) {
        self.cdx
            .get_or_insert_with(|| TimestampedCdxBalance::new(YoctoCdx(0)))
            .credit(credit)
    }

    pub fn apply_cdx_debit(&mut self, debit: YoctoCdx) {
        let balance = self.cdx.as_mut().expect("account has zero CDX balance");
        assert!(
            balance.amount() >= debit,
            "account CDX balance is too low to fulfill request"
        );
        balance.debit(debit);
        if balance.amount() == 0.into() {
            self.cdx = None
        }
    }
}

pub struct RegisteredAccount {
    pub account: Account,
    pub id: Hash,
}

impl Deref for RegisteredAccount {
    type Target = Account;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

impl DerefMut for RegisteredAccount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account
    }
}

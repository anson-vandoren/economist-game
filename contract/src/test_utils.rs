use crate::near::*;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{json_types::ValidAccountId, AccountId, VMContext};
use std::convert::TryInto;

pub fn to_valid(account: AccountId) -> ValidAccountId {
    account.try_into().expect("invalid account")
}

pub fn new_context(predecessor_account_id: &str) -> VMContext {
    VMContextBuilder::new()
        .current_account_id(to_valid("game.economist.near".to_string()))
        .signer_account_id(to_valid(predecessor_account_id.to_string()))
        .account_balance(1000 * YOCTO)
        .build()
}

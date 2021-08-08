pub mod core;
pub mod domain;
pub mod near;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, PanicOnDefault};

use crate::{
    core::Hash,
    domain::{Account, StorageUsage, TimestampedCdxBalance, YoctoNear},
    near::storage_keys::StorageKeys,
};
use near_sdk::json_types::ValidAccountId;

mod interface;
#[cfg(test)]
pub(crate) mod test_utils;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,

    /// bytes of storage needed to store each account
    account_storage_usage: StorageUsage,

    /// total (yocto)NEAR that this contract is holding in escrow in behalf of users
    /// to keep their accounts stored
    total_account_storage_escrow: YoctoNear,

    accounts: LookupMap<Hash, Account>,
    accounts_len: u128,

    total_cdx: TimestampedCdxBalance,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "contract is already initialized");
        assert_ne!(
            env::current_account_id().as_str(),
            owner_id.as_ref(),
            "contract cannot be owner"
        );

        let mut contract = Self {
            owner_id: owner_id.into(),

            total_account_storage_escrow: 0.into(),
            account_storage_usage: 0.into(), // computed after contract is created - see below

            accounts: LookupMap::new(StorageKeys::Accounts),
            accounts_len: 0,

            total_cdx: TimestampedCdxBalance::new(0.into()),
        };

        // compute account storage usage
        {
            let initial_storage_usage = env::storage_usage();
            contract.allocate_account_template_to_measure_storage_usage();
            contract.account_storage_usage =
                StorageUsage(env::storage_usage() - initial_storage_usage);
            contract.deallocate_account_template_to_measure_storage_usage();
            assert_eq!(initial_storage_usage, env::storage_usage());
        }

        contract
    }
}

impl Contract {
    /// this is used to compute the storage fees to charge for an account registration
    fn allocate_account_template_to_measure_storage_usage(&mut self) {
        let hash = Hash::from([0u8; 32]);
        let account_template = Account::account_template_to_measure_storage_usage();
        self.accounts.insert(&hash, &account_template);

        // if anything else can add to the size of the accounts storage, it should be initialized
        // here
    }

    fn deallocate_account_template_to_measure_storage_usage(&mut self) {
        let hash = Hash::from([0u8; 32]);
        self.accounts.remove(&hash);

        // clean up anything else that was instantiated to measure account storage
    }
}

#[cfg(test)]
mod tests {}

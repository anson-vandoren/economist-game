use near_sdk::{
    json_types::{ValidAccountId, U128},
    serde::{Deserialize, Serialize},
    Promise, PromiseOrValue,
};
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    ops::Deref,
};

pub trait FungibleToken {
    /// Simple transfer to a receiver.
    ///
    /// Requirements:
    /// - Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// - Caller must have greater than or equal to the `amount` being requested
    /// - Both accounts must be registered with the contract
    ///
    /// Arguments:
    /// - `receiver_id`: the valid NEAR account receiving the fungible tokens.
    /// - `amount`: the number of tokens to transfer, wrapped in quotes and treated
    ///   like a string, although the number will be stored as an unsigned integer
    ///   with 128 bits.
    /// - `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer.
    ///
    /// ## Panics:
    /// - if the attached deposit does not equal 1 yoctoNEAR
    /// - if either sender or receiver accounts are not registered
    /// - if amount is zero
    /// - if the sender account has insufficient funds to fulfill the request
    ///
    /// GAS REQUIREMENTS: 10 TGas
    /// #\[payable\]
    fn ft_transfer(&mut self, receiver_id: ValidAccountId, amount: TokenAmount, memo: Option<Memo>);
}

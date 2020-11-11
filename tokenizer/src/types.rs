use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey, Balance, EpochHeight};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

const NO_DEPOSIT: u128 = 0;

const SINGLE_CALL_GAS: u64 = 200_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WithdrawAccount {
    // Claim 한 시점에서 토큰이 소각되고, 해당 balance만큼 기록됨
    pub claim_balance: Balance,

    // Near withdraw 가능한 시간 기록
    pub unstaked_available_epoch_height: EpochHeight,
}

impl WithdrawAccount {
    pub fn new() -> Self {
        Self {
            claim_balance: 0,
            unstaked_available_epoch_height: 0,
        }
    }
}
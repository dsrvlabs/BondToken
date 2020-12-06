use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{Balance, EpochHeight};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

// Near supported 1e24
pub const DECIMAL: u128 = 1_000_000_000_000_000_000_000_000;

pub const NO_DEPOSIT: u128 = 0;

pub const SINGLE_CALL_GAS: u64 = 200_000_000_000_000;

pub const DEPOSIT_AND_STAKING_GAS: u64 = 100_000_000_000_000;

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
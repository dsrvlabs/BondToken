use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{AccountId};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    pub validator_info: UnorderedMap<AccountId, u32>,
    pub validator_count: u32,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            validator_info: UnorderedMap::new(b"a".to_vec()),
            validator_count: 0
        }
    }

    pub fn add_validator(&mut self, validator: AccountId, ratio: u32) {
        if self.validator_info.insert(&validator, &ratio).is_none() {
            self.validator_count += 1;
        } else {
            env::panic(b"Registry: Already exist Validator");
        }
    }

    pub fn del_validator(&mut self, validator: AccountId) {
        if self.validator_info.remove(&validator).is_some() {
            self.validator_count -= 1;
        } else {
            env::panic(b"Registry: Non-exist Validator");
        }
    }

    pub fn update_validator(&mut self, validator: AccountId, ratio: u32) {
        if self.validator_info.insert(&validator, &ratio).is_none() {
            env::panic(b"Registry: Non-exist Validator");
        }
    }

    pub fn get_validators(&self) -> Vec<(AccountId, u32)> {
        self.validator_info.to_vec()
    }

    pub fn get_validator_ratio(&self, validator: AccountId) -> Option<u32> {
        self.validator_info.get(&validator)
    }
}
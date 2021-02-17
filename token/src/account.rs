use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, AccountId, Balance
};

/// Contains balance and allowances information for one account.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub credit: U128,
    pub allowances: LookupMap<Vec<u8>, Balance>,
    pub allowances_count: u32,
}

impl Account {
    pub fn new(account_hash: Vec<u8>) -> Self {
        Self {
            credit: U128::from(0),
            allowances: LookupMap::new(account_hash),
            allowances_count: 0
        }
    }

    pub fn set_allowance(&mut self, escrow_account_id: &AccountId, allowance: Balance) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        if allowance > 0 {
            if self.allowances.insert(&escrow_hash, &allowance).is_none() {
                self.allowances_count += 1;
            }
        } else {
            if self.allowances.remove(&escrow_hash).is_some() {
                self.allowances_count -= 1;
            }
        }
    }

    pub fn get_allowance(&self, escrow_account_id: &AccountId) -> Balance {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        self.allowances.get(&escrow_hash).unwrap_or(0)
    }
}
use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey};

#[near_bindgen]
impl Tokenizer {
    #[payable]
    pub fn deposit(&self) {
        let owner_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        let mut total_ratio;
        for tuple in self.registry.get_validators().iter() {
            total_ratio += tuple.1;
        }

        for tuple in self.registry.get_validators().iter() {
            let mut amount * (tuple.1.into() / total_ratio);
            validator::ext_validator::deposit_and_stake(&tuple.0, amount, DEPOSIT_AND_STAKING_GAS);
        }
    }
}
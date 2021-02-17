use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey};

#[near_bindgen]
impl Tokenizer {
    /// only Governance
    /// add Validator with Ratio using validator AccoundId, ratio u32
    pub fn add_validator(&mut self, validator: AccountId, ratio: u32) {
        self.assert_governance();
        self.registry.add_validator(validator, ratio);
    }

    /// only Governance
    /// remove Validator using validator AccoundId
    pub fn remove_validator(&mut self, validator: AccountId) {
        self.assert_governance();
        self.registry.del_validator(validator);
    }

    /// only Governance
    /// update Validator with Ratio using validator AccoundId, ratio u32
    pub fn update_validator(&mut self, validator: AccountId, ratio: u32) {
        self.assert_governance();
        self.registry.update_validator(validator, ratio);
    }
}
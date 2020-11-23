use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey, Balance};

#[near_bindgen]
impl ScaleToken {
    ///@TODO: callback deposit
    /// mint(deposit balance mul * scale factor) -> transfer,
    pub fn on_deposit(&mut self, amount: U128, from: AccountId) {
        
    }

    ///@TODO: callback ping
    /// update scale factor
    pub fn on_ping(&mut self, total_amount: U128) {

    }
}
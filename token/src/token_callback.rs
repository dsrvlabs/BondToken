use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey, Balance};

#[near_bindgen]
impl ScaleToken {
    ///@TODO: callback deposit
    /// mint(deposit balance mul * scale factor) -> transfer,
    pub fn on_deposit(&mut self, mint_amount: U128, from: AccountId) {
        let credit = mint_amount.0 * self.scale_factor;
        self.mint_to(mint_amount, from);

        
    }

    ///@TODO: callback ping
    /// update scale factor
    pub fn on_ping(&mut self, new_total_amount: U128) {
        let now_scale_factor = self.scale_factor;
        let now_total_amount = self.total_credit / now_scale_factor;

        //
        std::cmp::min(now_total_amount, new_total_amount.0) / std::cmp::max(now_total_amount, new_total_amount.0)
    }
} 
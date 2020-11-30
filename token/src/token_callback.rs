use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey, Balance};

#[near_bindgen]
impl ScaleToken {
    pub fn on_deposit(&mut self, mint_amount: Balance, from: AccountId) {
        let credit = mint_amount.0 * self.scale_factor;
        /// make credit
        self.mint_to(credit, from);
    }

    pub fn on_withdraw(&mut self, burn_amount: Balance, from: AccountId) {
        let credit = mint_amount.0 * self.scale_factor;
        /// burn credit
        self.burn_from(credit, from);
    }

    /// update scale factor
    pub fn on_ping(&mut self, new_total_amount: U128) {
        let now_scale_factor = self.scale_factor;
        let now_total_amount = self.total_credit / now_scale_factor;
        self.scale_factor = (std::cmp::min(now_total_amount, new_total_amount.0) /
            std::cmp::max(now_total_amount, new_total_amount.0))
            * 10u128.pow(24);
    }
} 
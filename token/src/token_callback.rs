use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey, Balance};

#[near_bindgen]
impl ScaleToken {
    pub fn on_deposit(&mut self, mint_amount: Balance, from: AccountId) {
        self.mint_to(mint_amount, from);
    }

    pub fn on_withdraw(&mut self, burn_amount: Balance, from: AccountId) {
        self.burn_from(burn_amount, from);
    }

    /// update scale factor
    pub fn on_ping(&mut self, total_stake_amount: U128) {
        let now_scale_factor = self.scale_factor;
        let now_total_amount = self.total_credit / now_scale_factor;
        let min = std::cmp::min(now_total_amount, total_stake_amount.0);
        let max = std::cmp::max(now_total_amount, total_stake_amount.0);
        self.scale_factor = self.devide_scale(min, max);
    }
} 
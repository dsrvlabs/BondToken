use near_sdk::ext_contract;
use near_sdk::json_types::U128;

#[ext_contract(ext_nep21)]
pub trait NEP21 {
    fn transfer(&mut self, dest: AccountId, amount: U128);

    fn transfer_from(&mut self, from: AccountId, dest: AccountId, amount: U128);

    fn mint_to(&mut self, amount: U128, target: AccountId);

    fn burn_from(&mut self, amount: Balance, target: AccountId);
}
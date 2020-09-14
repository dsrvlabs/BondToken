use near_sdk::ext_contract;
use near_sdk::json_types::U128;

#[ext_contract(ext_validator)]
pub trait validator {
    // #[payable]
    fn transfer(&mut self, dest: AccountId, amount: U128);

    // #[payable]
    fn transfer_from(&mut self, from: AccountId, dest: AccountId, amount: U128);
}
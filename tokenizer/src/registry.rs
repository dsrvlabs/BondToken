use near_sdk::ext_contract;
use near_sdk::json_types::U128;

#[ext_contract(ext_registry)]
pub trait Registry {
    /// get Registred Validators
    pub fn get_validators(self) -> LookupMap<String, u32>;
}
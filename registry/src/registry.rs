use near_sdk::ext_contract;

#[ext_contract(ext_registry)]
pub trait Registry {
    /// only Governance
    /// add Validator Information
    fn add_validator(&mut self, validator: AccountId, ratio: u32);

    /// only Governance
    /// delete Validator Information using validator public key
    fn del_validator(&mut self, validator: AccountId);

    /// only Governance
    /// update Validator Ratio
    fn update_validator(&mut self, validator: AccountId, ratio: u32);

    /// get Validator Information vector
    fn get_validators(&self) -> Vec<(AccountId, u32)>;

    /// get Validator ratio using validator public key
    fn get_validator_ratio(&self, validator: AccountId) -> Option<u32>;
}
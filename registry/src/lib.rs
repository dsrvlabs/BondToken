use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{
    env, near_bindgen, AccountId
};

pub mod registry;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

// @TODO: Dynamic Ratio
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    pub governance: AccountId,
    pub validator_info: UnorderedMap<AccountId, u32>,
    pub validator_count: u32,
}

impl Default for Registry {
    fn default() -> Self {
        panic!("Registry should be initialized before usage")
    }
}

#[near_bindgen]
impl Registry {
    #[init]
    pub fn new(governance: AccountId) -> Self {
        assert!(!env::state_exists(), "Registry: Already initialized");
        Self {
            governance,
            validator_info: UnorderedMap::new(b"a".to_vec()),
            validator_count: 0
        }
    }

    pub fn add_validator(&mut self, validator: AccountId, ratio: u32) {
        let caller = env::predecessor_account_id();
        if caller != self.governance {
            env::panic(b"Registry: Caller is not Governance");
        }
        if self.validator_info.insert(&validator, &ratio).is_none() {
            self.validator_count += 1;
        } else {
            env::panic(b"Registry: Already exist Validator");
        }
    }

    pub fn del_validator(&mut self, validator: AccountId) {
        let caller = env::predecessor_account_id();
        if caller != self.governance {
            env::panic(b"Registry: Caller is not Governance");
        }
        if self.validator_info.remove(&validator).is_some() {
            self.validator_count -= 1;
        } else {
            env::panic(b"Registry: Non-exist Validator");
        }
    }

    pub fn update_validator(&mut self, validator: AccountId, ratio: u32) {
        let caller = env::predecessor_account_id();
        if caller != self.governance {
            env::panic(b"Registry: Caller is not Governance");
        }
        if self.validator_info.insert(&validator, &ratio).is_none() {
            env::panic(b"Registry: Non-exist Validator");
        }
    }

    pub fn get_validators(&self) -> Vec<(AccountId, u32)> {
        self.validator_info.to_vec()
    }

    pub fn get_validator_ratio(&self, validator: AccountId) -> Option<u32> {
        self.validator_info.get(&validator)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    /// Governance Account
    fn governance() -> AccountId {
        "governance.near".to_string()
    }

    fn deployer() -> AccountId {
        "deployer.near".to_string()
    }

    // start Validator List
    fn alice() -> AccountId {
        "alice.near".to_string()
    }

    // fn bob() -> AccountId {
    //     "bob.near".to_string()
    // }

    // fn carol() -> AccountId {
    //     "carol.near".to_string()
    // }
    // end of Validator List

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: governance(),
            signer_account_id: governance(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1_000_000_000_000_000_000_000_000_000u128,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_initialize_new_registry() {
        let context = get_context(deployer());
        testing_env!(context);
        let contract = Registry::new(governance());
        assert_eq!(contract.validator_count, 0);
    }

    #[test]
    fn test_add_validator_with_get_ratio() {
        let _ratio: u32 = 10;

        let mut context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        context.predecessor_account_id = governance();
        testing_env!(context.clone());
        contract.add_validator(alice(), _ratio);

        assert_eq!(contract.validator_count, 1);
        assert_eq!(contract.get_validator_ratio(alice()), Some(_ratio));
    }

    #[test]
    #[should_panic(expected = "Registry: Caller is not Governance")]
    fn test_add_validator_with_non_governance_call() {
        let _ratio: u32 = 10;

        let context = get_context(deployer());
        testing_env!(context);

        let mut contract = Registry::new(governance());
        contract.add_validator(alice(), _ratio);
    }

    #[test]
    fn test_add_validator_with_get_all_validators() {
        let _ratio: u32 = 10;

        let mut context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        context.predecessor_account_id = governance();
        testing_env!(context.clone());
        contract.add_validator(alice(), _ratio);
        assert_eq!(contract.validator_count, 1);
        assert_eq!(contract.get_validators(), vec![(alice(), _ratio)]);
    }

    #[test]
    fn test_del_validator() {
        let _ratio: u32 = 10;

        let mut context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        context.predecessor_account_id = governance();
        testing_env!(context.clone());

        contract.add_validator(alice(), _ratio);
        contract.del_validator(alice());
        assert_eq!(contract.validator_count, 0);
        assert_eq!(contract.get_validators(), vec![]);
    }

    #[test]
    #[should_panic(expected = "Registry: Caller is not Governance")]
    fn test_del_validator_with_non_governance_call() {
        let context = get_context(deployer());
        testing_env!(context);

        let mut contract = Registry::new(governance());
        contract.del_validator(alice());
    }

    #[test]
    #[should_panic(expected = "Registry: Non-exist Validator")]
    fn test_del_validator_with_non_registred_validators() {
        let mut context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        context.predecessor_account_id = governance();
        testing_env!(context.clone());
        contract.del_validator(alice());
    }

    #[test]
    fn test_update_validator() {
        let _ratio = 10u32;
        let mut context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        context.predecessor_account_id = governance();
        testing_env!(context.clone());

        contract.add_validator(alice(), _ratio);
        contract.update_validator(alice(), _ratio + 11u32);

        assert_eq!(contract.get_validator_ratio(alice()), Some(21u32));
    }

    #[test]
    #[should_panic(expected = "Registry: Caller is not Governance")]
    fn test_update_validator_with_non_governance_call() {
        let _ratio = 10u32;
        let context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        testing_env!(context.clone());

        contract.update_validator(alice(), _ratio + 11u32);
    }

    #[test]
    #[should_panic(expected = "Registry: Non-exist Validator")]
    fn test_update_validator_with_non_exist_validator() {
        let _ratio = 10u32;
        let mut context = get_context(deployer());
        testing_env!(context.clone());

        let mut contract = Registry::new(governance());
        context.predecessor_account_id = governance();
        testing_env!(context.clone());

        contract.update_validator(alice(), _ratio + 11u32);

        // assert_eq!(contract.get_validator_ratio(alice()), Some(21u32));
    }

    #[test]
    fn test_get_validator_ratio_with_non_exist() {
        let context = get_context(deployer());
        testing_env!(context.clone());

        let contract = Registry::new(governance());
        assert_eq!(contract.get_validator_ratio(alice()), None);
    }
}
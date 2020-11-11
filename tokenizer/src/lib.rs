use near_sdk::ext_contract;
use near_sdk::collections::{LookupMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, AccountId, Balance
};

pub use crate::registry::{Registry};
pub use crate::types::*;
pub use crate::internal::*;

pub mod registry;
pub mod types;
pub mod internal;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[ext_contract(ext_validator)]
pub trait ExtValidator {
    /// Deposits the attached amount into the inner account of the predecessor.
    // #[payable]
    fn deposit(&mut self);

    /// Deposits the attached amount into the inner account of the predecessor and stakes it.
    // #[payable]
    fn deposit_and_stake(&mut self);

    /// Withdraws the entire unstaked balance from the predecessor account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    fn withdraw_all(&mut self);

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    fn withdraw(&mut self, amount: U128);

    /// Stakes all available unstaked balance from the inner account of the predecessor.
    fn stake_all(&mut self);

    /// Stakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough unstaked balance.
    fn stake(&mut self, amount: U128);

    /// Unstakes all staked balance from the inner account of the predecessor.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    fn unstake_all(&mut self);

    /// Unstakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough staked balance.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    fn unstake(&mut self, amount: U128);
}

#[ext_contract(ext_ft)]
pub trait ExtFT {
    fn transfer(&mut self, dest: AccountId, amount: U128);

    fn transfer_from(&mut self, from: AccountId, dest: AccountId, amount: U128);

    fn mint_to(&mut self, amount: u128, target: AccountId) -> U128;
    
    fn burn_from(&mut self, amount: u128, target: AccountId);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokenizer {
    token: AccountId,
    /// for withdraws
    /// sha256(AccountId) -> Withdrawer Detail
    withdraws: LookupMap<Vec<u8>, WithdrawAccount>,
    governance: AccountId,
    registry: Registry,
    total_waiting: Balance
}

impl Default for Tokenizer {
    fn default() -> Self {
        panic!("Tokenizer should be initialized before usage")
    }
}

#[near_bindgen]
impl Tokenizer {
    #[init]
    pub fn new(governance: AccountId) -> Self {
        assert!(!env::state_exists(), "Registry: Already initialized");
        Self {
            token: "".to_string(),
            withdraws: LookupMap::new(b"a".to_vec()),
            governance,
            registry: Registry::new(),
            total_waiting: 0
        }
    }
}


// use token::{nep21};
// use registry::{registry};
// use token::ScaleToken;
// pub mod utils;
// mod validator;


// const DEPOSIT_AND_STAKING_GAS: u64 = 100_000_000_000_000;
// const STORAGE_PRICE_PER_BYTE: Balance = 100_000_000_000_000_000_000;
// const SINGLE_CALL_GAS: u64 = 200_000_000_000_000;




// #[near_bindgen]
// impl Tokenizer {

//     #[init]
//     pub fn new(governance: AccountId, registry: AccountId) -> Self {
//         assert!(!env::state_exists(), "Registry: Already initialized");
//         Self {
//             token: "".to_string(),
//             governance,
//             registry: Registry::new()
//         }
//     }

//     pub fn set_token(&mut self, token: AccountId) {
//         let caller = env::predecessor_account_id();
//         if caller != self.governance {
//             env::panic(b"Caller is not Governance");
//         }
//         self.token = token;
//     }

//     pub fn deposit(&self) {
//         let owner_id = env::predecessor_account_id();
//         let deposit = env::attached_deposit();

//         let receipt = env::promise_create(
//             self.registry.clone(),
//             b"get_validators",
//             &[],
//             0,
//             SINGLE_CALL_GAS,
//         );

//         let validators_bytes = match env::promise_result(receipt) {
//             PromiseResult::Successful(x) => x,
//             PromiseResult::Failed => env::panic(b"The promise failed. See receipt failures."),
//             PromiseResult::NotReady => env::panic(b"The promise was not ready."),
//         };

//         let validator_info = Vec<>::From(&validators_bytes);

//         for (validator, ratio) in validator_info {
//             let amount = (deposit / 10000) * ratio;
//             println!("{:?}, {:?}", validator, amount);
//             // validator::ext_validator::deposit_and_stake(&validator, amount, DEPOSIT_AND_STAKING_GAS);
//         }
//     }

//     // fn deposit_and_stake(&mut self, validator: AccountId, amount: Balance) {
//     //     validator::ext_validator::deposit_and_stake(&validator, amount, DEPOSIT_AND_STAKING_GAS);
//     //     env::log(format!("{} to staking amount {}", validator, amount).as_bytes());
//     // }

//     // #[result_serializer(borsh)]
//     // fn validators(&self) -> Promise {
//     //     registry::ext_registry::get_validators(&self.registry, 0, DEPOSIT_AND_STAKING_GAS)
//     // }
// }

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     use super::*;

//     /// Governance Account
//     fn governance() -> AccountId {
//         "governance.near".to_string()
//     }

//     fn deployer() -> AccountId {
//         "deployer.near".to_string()
//     }

//     // start Validator List
//     fn alice() -> AccountId {
//         "alice.near".to_string()
//     }

//     // fn bob() -> AccountId {
//     //     "bob.near".to_string()
//     // }

//     // fn carol() -> AccountId {
//     //     "carol.near".to_string()
//     // }
//     // end of Validator List

//     fn get_context(predecessor_account_id: AccountId) -> VMContext {
//         VMContext {
//             current_account_id: governance(),
//             signer_account_id: governance(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id,
//             input: vec![],
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 1_000_000_000_000_000_000_000_000_000u128,
//             account_locked_balance: 0,
//             storage_usage: 10u64.pow(6),
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view: false,
//             output_data_receivers: vec![],
//             epoch_height: 0,
//         }
//     }

//     // #[test]
//     // fn test_intialize_new_tokenizer() {
//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let registry = Registry::new(governance());
//     //     let tokenizer = Tokenizer::new(governance(), registry);
//     // }

//     // #[test]
//     // fn test_initialize_new_registry() {
//     //     let context = get_context(deployer());
//     //     testing_env!(context);
//     //     let contract = Registry::new(governance());
//     //     assert_eq!(contract.validator_count, 0);
//     // }

//     // #[test]
//     // fn test_add_validator_with_get_ratio() {
//     //     let _ratio: u32 = 10;

//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     context.predecessor_account_id = governance();
//     //     testing_env!(context.clone());
//     //     contract.add_validator(alice(), _ratio);

//     //     assert_eq!(contract.validator_count, 1);
//     //     assert_eq!(contract.get_validator_ratio(alice()), Some(_ratio));
//     // }

//     // #[test]
//     // #[should_panic(expected = "Registry: Caller is not Governance")]
//     // fn test_add_validator_with_non_governance_call() {
//     //     let _ratio: u32 = 10;

//     //     let context = get_context(deployer());
//     //     testing_env!(context);

//     //     let mut contract = Registry::new(governance());
//     //     contract.add_validator(alice(), _ratio);
//     // }

//     // #[test]
//     // fn test_add_validator_with_get_all_validators() {
//     //     let _ratio: u32 = 10;

//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     context.predecessor_account_id = governance();
//     //     testing_env!(context.clone());
//     //     contract.add_validator(alice(), _ratio);
//     //     assert_eq!(contract.validator_count, 1);
//     //     assert_eq!(contract.get_validators(), vec![(alice(), _ratio)]);
//     // }

//     // #[test]
//     // fn test_del_validator() {
//     //     let _ratio: u32 = 10;

//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     context.predecessor_account_id = governance();
//     //     testing_env!(context.clone());

//     //     contract.add_validator(alice(), _ratio);
//     //     contract.del_validator(alice());
//     //     assert_eq!(contract.validator_count, 0);
//     //     assert_eq!(contract.get_validators(), vec![]);
//     // }

//     // #[test]
//     // #[should_panic(expected = "Registry: Caller is not Governance")]
//     // fn test_del_validator_with_non_governance_call() {
//     //     let context = get_context(deployer());
//     //     testing_env!(context);

//     //     let mut contract = Registry::new(governance());
//     //     contract.del_validator(alice());
//     // }

//     // #[test]
//     // #[should_panic(expected = "Registry: Non-exist Validator")]
//     // fn test_del_validator_with_non_registred_validators() {
//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     context.predecessor_account_id = governance();
//     //     testing_env!(context.clone());
//     //     contract.del_validator(alice());
//     // }

//     // #[test]
//     // fn test_update_validator() {
//     //     let _ratio = 10u32;
//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     context.predecessor_account_id = governance();
//     //     testing_env!(context.clone());

//     //     contract.add_validator(alice(), _ratio);
//     //     contract.update_validator(alice(), _ratio + 11u32);

//     //     assert_eq!(contract.get_validator_ratio(alice()), Some(21u32));
//     // }

//     // #[test]
//     // #[should_panic(expected = "Registry: Caller is not Governance")]
//     // fn test_update_validator_with_non_governance_call() {
//     //     let _ratio = 10u32;
//     //     let context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     testing_env!(context.clone());

//     //     contract.update_validator(alice(), _ratio + 11u32);
//     // }

//     // #[test]
//     // #[should_panic(expected = "Registry: Non-exist Validator")]
//     // fn test_update_validator_with_non_exist_validator() {
//     //     let _ratio = 10u32;
//     //     let mut context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let mut contract = Registry::new(governance());
//     //     context.predecessor_account_id = governance();
//     //     testing_env!(context.clone());

//     //     contract.update_validator(alice(), _ratio + 11u32);

//     //     // assert_eq!(contract.get_validator_ratio(alice()), Some(21u32));
//     // }

//     // #[test]
//     // fn test_get_validator_ratio_with_non_exist() {
//     //     let context = get_context(deployer());
//     //     testing_env!(context.clone());

//     //     let contract = Registry::new(governance());
//     //     assert_eq!(contract.get_validator_ratio(alice()), None);
//     // }
// }
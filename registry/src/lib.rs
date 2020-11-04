use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{
    env, near_bindgen, AccountId
};

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
    pub fn new (governance: AccountId) -> Self {
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

    // #[test]
    // #[should_panic]
    // fn test_initialize_new_token_twice_fails() {
    //     let context = get_context(carol());
    //     testing_env!(context);
    //     {
    //         let _contract = ScaleToken::new(tokenizer());
    //     }
    //     ScaleToken::new(tokenizer());
    // }

    // #[test]
    // fn test_transfer_to_a_different_account_works() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.storage_usage = env::storage_usage();
    //     context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
    //     testing_env!(context.clone());
    //     let transfer_amount = mint_balance / 3;
    //     contract.transfer(bob(), transfer_amount.into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();

    //     context.is_view = true;
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     assert_eq!(
    //         contract.get_balance(carol()).0,
    //         (mint_balance - transfer_amount)
    //     );
    //     assert_eq!(contract.get_balance(bob()).0, transfer_amount);
    // }

    // #[test]
    // #[should_panic(expected = "The new owner should be different from the current owner")]
    // fn test_transfer_to_self_fails() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.storage_usage = env::storage_usage();
    //     context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
    //     testing_env!(context.clone());
    //     let transfer_amount = mint_balance / 3;
    //     contract.transfer(carol(), transfer_amount.into());
    // }

    // #[test]
    // #[should_panic(expected = "Can not increment allowance for yourself")]
    // fn test_increment_allowance_to_self_fails() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(carol(), (mint_balance / 2).into());
    // }

    // #[test]
    // #[should_panic(expected = "Can not decrement allowance for yourself")]
    // fn test_decrement_allowance_to_self_fails() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(carol(), (mint_balance / 2).into());
    // }

    // #[test]
    // fn test_decrement_allowance_after_allowance_was_saturated() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), (mint_balance / 2).into());
    //     assert_eq!(contract.get_allowance(carol(), bob()), 0.into())
    // }

    // #[test]
    // fn test_increment_allowance_does_not_overflow() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), (mint_balance * 2).into());
    //     assert_eq!(
    //         contract.get_allowance(carol(), bob()),
    //         std::u128::MAX.into()
    //     )
    // }

    // #[test]
    // #[should_panic(
    //     expected = "The required attached deposit is 12400000000000000000000, but the given attached deposit is is 0"
    // )]
    // fn test_increment_allowance_with_insufficient_attached_deposit() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), (mint_balance / 2).into());
    // }

    // #[test]
    // fn test_carol_escrows_to_bob_transfers_to_alice() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.storage_usage = env::storage_usage();

    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert_eq!(contract.get_total_supply().0, mint_balance);

    //     let allowance = mint_balance / 3;
    //     let transfer_amount = allowance / 3;
    //     context.is_view = false;
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), allowance.into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();

    //     context.is_view = true;
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);

    //     // Acting as bob now
    //     context.is_view = false;
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     context.predecessor_account_id = bob();
    //     testing_env!(context.clone());
    //     contract.transfer_from(carol(), alice(), transfer_amount.into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();

    //     context.is_view = true;
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     assert_eq!(
    //         contract.get_balance(carol()).0,
    //         mint_balance - transfer_amount
    //     );
    //     assert_eq!(contract.get_balance(alice()).0, transfer_amount);
    //     assert_eq!(
    //         contract.get_allowance(carol(), bob()).0,
    //         allowance - transfer_amount
    //     );
    // }

    // #[test]
    // fn test_carol_escrows_to_bob_locks_and_transfers_to_alice() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();

    //     context.storage_usage = env::storage_usage();

    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert_eq!(contract.get_total_supply().0, mint_balance);

    //     let allowance = mint_balance / 3;
    //     let transfer_amount = allowance / 3;
    //     context.is_view = false;
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), allowance.into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();

    //     context.is_view = true;
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);
    //     assert_eq!(contract.get_balance(carol()).0, mint_balance);

    //     // Acting as bob now
    //     context.is_view = false;
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     context.predecessor_account_id = bob();
    //     testing_env!(context.clone());
    //     contract.transfer_from(carol(), alice(), transfer_amount.into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();

    //     context.is_view = true;
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     assert_eq!(
    //         contract.get_balance(carol()).0,
    //         (mint_balance - transfer_amount)
    //     );
    //     assert_eq!(contract.get_balance(alice()).0, transfer_amount);
    //     assert_eq!(
    //         contract.get_allowance(carol(), bob()).0,
    //         allowance - transfer_amount
    //     );
    // }

    // #[test]
    // fn test_self_allowance_set_for_refund() {
    //     let mut context = get_context(tokenizer());
    //     testing_env!(context.clone());
    //     let mint_balance = 1_000_000_000_000_000u128;
    //     let mut contract = ScaleToken::new(tokenizer());
    //     contract.mint_to(mint_balance, carol());

    //     context.predecessor_account_id = carol();
    //     context.storage_usage = env::storage_usage();

    //     let initial_balance = context.account_balance;
    //     let initial_storage = context.storage_usage;
    //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), (mint_balance / 2).into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();
    //     assert_eq!(
    //         context.account_balance,
    //         initial_balance
    //             + Balance::from(context.storage_usage - initial_storage) * STORAGE_PRICE_PER_BYTE
    //     );

    //     let initial_balance = context.account_balance;
    //     let initial_storage = context.storage_usage;
    //     testing_env!(context.clone());
    //     context.attached_deposit = 0;
    //     testing_env!(context.clone());
    //     contract.approve(bob(), (mint_balance / 2).into());
    //     context.storage_usage = env::storage_usage();
    //     context.account_balance = env::account_balance();
    //     assert!(context.storage_usage == initial_storage);
    //     assert!(context.account_balance == initial_balance);
    //     assert_eq!(
    //         context.account_balance,
    //         initial_balance
    //             - Balance::from(initial_storage - context.storage_usage) * STORAGE_PRICE_PER_BYTE
    //     );
    // }
}
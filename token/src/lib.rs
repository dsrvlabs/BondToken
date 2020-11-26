use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise, StorageUsage
};

pub use crate::token::*;
pub use crate::account::*;
pub use crate::internal::*;

pub mod token;
pub mod account;
pub mod internal;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

/// Price per 1 byte of storage from mainnet genesis config.
const STORAGE_PRICE_PER_BYTE: Balance = 100_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ScaleToken {
    /// sha256(AccountID) -> Account details.
    pub accounts: LookupMap<Vec<u8>, Account>,

    /// Total supply of the all token.
    pub total_credit: Balance,

    /// 토큰이 제공하는 소수점자리 18u8
    pub decimals: u8,

    /// Tokenizer
    pub tokenizer: AccountId,

    /// default 
    pub scale_factor: Balance,
}

impl Default for ScaleToken {
    fn default() -> Self {
        panic!("ScaleToken should be initialized before usage")
    }
}

#[near_bindgen]
impl ScaleToken {
    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    #[init]
    pub fn new(tokenizer_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            accounts: LookupMap::new(b"a".to_vec()),
            total_credit: 0u128.into(),
            decimals: 18u8,
            tokenizer: tokenizer_id,
            scale_factor: 10u128.pow(24).into()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    fn tokenizer() -> AccountId {
        "tokenizer.near".to_string()
    }
    fn alice() -> AccountId {
        "alice.near".to_string()
    }
    fn bob() -> AccountId {
        "bob.near".to_string()
    }
    fn carol() -> AccountId {
        "carol.near".to_string()
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: bob(),
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
    fn test_initialize_new_token() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 0u128;
        let contract = ScaleToken::new(tokenizer());
        assert_eq!(contract.get_total_supply().0, total_supply);
        assert_eq!(contract.get_balance(tokenizer()).0, total_supply);
    }

    #[test]
    fn test_transfer_to_a_different_account_works() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance.into(), carol());

        context.predecessor_account_id = carol();
        context.storage_usage = env::storage_usage();
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        let transfer_amount = mint_balance / 3;
        contract.transfer(bob(), transfer_amount.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(
            contract.get_balance(carol()).0,
            (mint_balance - transfer_amount)
        );
        assert_eq!(contract.get_balance(bob()).0, transfer_amount);
    }
}

//     // #[test]
//     // #[should_panic]
//     // fn test_initialize_new_token_twice_fails() {
//     //     let context = get_context(carol());
//     //     testing_env!(context);
//     //     {
//     //         let _contract = ScaleToken::new(tokenizer());
//     //     }
//     //     ScaleToken::new(tokenizer());
//     // }

//     #[test]
//     fn test_transfer_to_a_different_account_works() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();
//         context.storage_usage = env::storage_usage();
//         context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
//         testing_env!(context.clone());
//         let transfer_amount = mint_balance / 3;
//         contract.transfer(bob(), transfer_amount.into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();

//         context.is_view = true;
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         assert_eq!(
//             contract.get_balance(carol()).0,
//             (mint_balance - transfer_amount)
//         );
//         assert_eq!(contract.get_balance(bob()).0, transfer_amount);
//     }

//     #[test]
//     #[should_panic(expected = "The new owner should be different from the current owner")]
//     fn test_transfer_to_self_fails() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();
//         context.storage_usage = env::storage_usage();
//         context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
//         testing_env!(context.clone());
//         let transfer_amount = mint_balance / 3;
//         contract.transfer(carol(), transfer_amount.into());
//     }

//     #[test]
//     #[should_panic(expected = "Can not increment allowance for yourself")]
//     fn test_increment_allowance_to_self_fails() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();
//         context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//         testing_env!(context.clone());
//         contract.inc_allowance(carol(), (mint_balance / 2).into());
//     }

//     // #[test]
//     // #[should_panic(expected = "Can not decrement allowance for yourself")]
//     // fn test_decrement_allowance_to_self_fails() {
//     //     let mut context = get_context(tokenizer());
//     //     testing_env!(context.clone());
//     //     let mint_balance = 1_000_000_000_000_000u128;
//     //     let mut contract = ScaleToken::new(tokenizer());
//     //     contract.mint_to(mint_balance, carol());

//     //     context.predecessor_account_id = carol();
//     //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//     //     testing_env!(context.clone());
//     //     contract.approve(carol(), (mint_balance / 2).into());
//     // }

//     // #[test]
//     // fn test_decrement_allowance_after_allowance_was_saturated() {
//     //     let mut context = get_context(tokenizer());
//     //     testing_env!(context.clone());
//     //     let mint_balance = 1_000_000_000_000_000u128;
//     //     let mut contract = ScaleToken::new(tokenizer());
//     //     contract.mint_to(mint_balance, carol());

//     //     context.predecessor_account_id = carol();
//     //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//     //     testing_env!(context.clone());
//     //     contract.approve(bob(), (mint_balance / 2).into());
//     //     assert_eq!(contract.get_allowance(carol(), bob()), 0.into())
//     // }

//     // #[test]
//     // fn test_increment_allowance_does_not_overflow() {
//     //     let mut context = get_context(tokenizer());
//     //     testing_env!(context.clone());
//     //     let mint_balance = 1_000_000_000_000_000u128;
//     //     let mut contract = ScaleToken::new(tokenizer());
//     //     contract.mint_to(mint_balance, carol());

//     //     context.predecessor_account_id = carol();
//     //     context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//     //     testing_env!(context.clone());
//     //     contract.approve(bob(), (mint_balance * 2).into());
//     //     assert_eq!(
//     //         contract.get_allowance(carol(), bob()),
//     //         std::u128::MAX.into()
//     //     )
//     // }

//     #[test]
//     #[should_panic(
//         expected = "The required attached deposit is 12400000000000000000000, but the given attached deposit is is 0"
//     )]
//     fn test_increment_allowance_with_insufficient_attached_deposit() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         contract.inc_allowance(bob(), (mint_balance / 2).into());
//     }

//     #[test]
//     fn test_carol_escrows_to_bob_transfers_to_alice() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();
//         context.storage_usage = env::storage_usage();

//         context.is_view = true;
//         testing_env!(context.clone());
//         assert_eq!(contract.get_total_supply().0, mint_balance);

//         let allowance = mint_balance / 3;
//         let transfer_amount = allowance / 3;
//         context.is_view = false;
//         context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//         testing_env!(context.clone());
//         contract.inc_allowance(bob(), allowance.into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();

//         context.is_view = true;
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);

//         // Acting as bob now
//         context.is_view = false;
//         context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//         context.predecessor_account_id = bob();
//         testing_env!(context.clone());
//         contract.transfer_from(carol(), alice(), transfer_amount.into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();

//         context.is_view = true;
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         assert_eq!(
//             contract.get_balance(carol()).0,
//             mint_balance - transfer_amount
//         );
//         assert_eq!(contract.get_balance(alice()).0, transfer_amount);
//         assert_eq!(
//             contract.get_allowance(carol(), bob()).0,
//             allowance - transfer_amount
//         );
//     }

//     #[test]
//     fn test_carol_escrows_to_bob_locks_and_transfers_to_alice() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();

//         context.storage_usage = env::storage_usage();

//         context.is_view = true;
//         testing_env!(context.clone());
//         assert_eq!(contract.get_total_supply().0, mint_balance);

//         let allowance = mint_balance / 3;
//         let transfer_amount = allowance / 3;
//         context.is_view = false;
//         context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//         testing_env!(context.clone());
//         contract.inc_allowance(bob(), allowance.into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();

//         context.is_view = true;
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);
//         assert_eq!(contract.get_balance(carol()).0, mint_balance);

//         // Acting as bob now
//         context.is_view = false;
//         context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//         context.predecessor_account_id = bob();
//         testing_env!(context.clone());
//         contract.transfer_from(carol(), alice(), transfer_amount.into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();

//         context.is_view = true;
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         assert_eq!(
//             contract.get_balance(carol()).0,
//             (mint_balance - transfer_amount)
//         );
//         assert_eq!(contract.get_balance(alice()).0, transfer_amount);
//         assert_eq!(
//             contract.get_allowance(carol(), bob()).0,
//             allowance - transfer_amount
//         );
//     }

//     #[test]
//     fn test_self_allowance_set_for_refund() {
//         let mut context = get_context(tokenizer());
//         testing_env!(context.clone());
//         let mint_balance = 1_000_000_000_000_000u128;
//         let mut contract = ScaleToken::new(tokenizer());
//         contract.mint_to(mint_balance.into(), carol());

//         context.predecessor_account_id = carol();
//         context.storage_usage = env::storage_usage();

//         let initial_balance = context.account_balance;
//         let initial_storage = context.storage_usage;
//         context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
//         testing_env!(context.clone());
//         contract.inc_allowance(bob(), (mint_balance / 2).into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();
//         assert_eq!(
//             context.account_balance,
//             initial_balance
//                 + Balance::from(context.storage_usage - initial_storage) * STORAGE_PRICE_PER_BYTE
//         );

//         let initial_balance = context.account_balance;
//         let initial_storage = context.storage_usage;
//         testing_env!(context.clone());
//         context.attached_deposit = 0;
//         testing_env!(context.clone());
//         contract.inc_allowance(bob(), (mint_balance / 2).into());
//         context.storage_usage = env::storage_usage();
//         context.account_balance = env::account_balance();
//         assert!(context.storage_usage == initial_storage);
//         assert!(context.account_balance == initial_balance);
//         assert_eq!(
//             context.account_balance,
//             initial_balance
//                 - Balance::from(initial_storage - context.storage_usage) * STORAGE_PRICE_PER_BYTE
//         );
//     }
// }
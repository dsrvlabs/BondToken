use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise, StorageUsage
};

pub mod nep21;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

//@TODO: Scale factor

/// Price per 1 byte of storage from mainnet genesis config.
const STORAGE_PRICE_PER_BYTE: Balance = 100_000_000_000_000_000_000;

/// Contains balance and allowances information for one account.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub balance: Balance,
    pub allowances: LookupMap<Vec<u8>, Balance>,
    pub allowances_count: u32,
}

impl Account {
    pub fn new(account_hash: Vec<u8>) -> Self {
        Self {
            balance: 0,
            allowances: LookupMap::new(account_hash),
            allowances_count: 0
        }
    }

    pub fn set_allowance(&mut self, escrow_account_id: &AccountId, allowance: Balance) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        if allowance > 0 {
            if self.allowances.insert(&escrow_hash, &allowance).is_none() {
                self.allowances_count += 1;
            }
        } else {
            if self.allowances.remove(&escrow_hash).is_some() {
                self.allowances_count -= 1;
            }
        }
    }

    pub fn get_allowance(&self, escrow_account_id: &AccountId) -> Balance {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        self.allowances.get(&escrow_hash).unwrap_or(0)
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ScaleToken {
    /// sha256(AccountID) -> Account details.
    pub accounts: LookupMap<Vec<u8>, Account>,

    /// Total supply of the all token.
    pub total_supply: Balance,

    /// 토큰이 제공하는 소수점자리 18u8
    pub decimals: u8,

    /// Tokenizer
    pub tokenizer: AccountId,
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
            total_supply: 0u128,
            decimals: 18u8,
            tokenizer: tokenizer_id
        }
    }

    pub fn decimals(&self) -> u8 {
        return self.decimals;
    }

    /// Returns total supply of tokens.
    pub fn get_total_supply(&self) -> U128 {
        self.total_supply.into()
    }

    /// Returns balance of the `owner_id` account.
    pub fn get_balance(&self, owner_id: AccountId) -> U128 {
        self.get_account(&owner_id).balance.into()
    }

    #[payable]
    pub fn approve(&mut self, escrow_account_id: AccountId, amount: U128) {
        let initial_storage = env::storage_usage();
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        let owner_id = env::predecessor_account_id();
        if escrow_account_id == owner_id {
            env::panic(b"Can not increment allowance for yourself");
        }
        let mut account = self.get_account(&owner_id);
        account.set_allowance(&escrow_account_id, amount.into());
        self.set_account(&owner_id, &account);
        self.refund_storage(initial_storage);
    }

    #[payable]
    pub fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, amount: U128) {
        let initial_storage = env::storage_usage();
        assert!(
            env::is_valid_account_id(new_owner_id.as_bytes()),
            "New owner's account ID is invalid"
        );
        let amount = amount.into();
        if amount == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
        assert_ne!(
            owner_id, new_owner_id,
            "The new owner should be different from the current owner"
        );
        // Retrieving the account from the state.
        let mut account = self.get_account(&owner_id);

        env::log(format!("transfer_from {} to {}, {}", owner_id, new_owner_id, amount).as_bytes());

        // Checking and updating unlocked balance
        if account.balance < amount {
            env::panic(b"Not enough balance");
        }
        account.balance -= amount;

        // If transferring by escrow, need to check and update allowance.
        let escrow_account_id = env::predecessor_account_id();
        if escrow_account_id != owner_id {
            let allowance = account.get_allowance(&escrow_account_id);
            if allowance < amount {
                env::panic(b"Not enough allowance");
            }
            account.set_allowance(&escrow_account_id, allowance - amount);
        }

        // Saving the account back to the state.
        self.set_account(&owner_id, &account);

        // Deposit amount to the new owner and save the new account to the state.
        let mut new_account = self.get_account(&new_owner_id);
        new_account.balance += amount;
        self.set_account(&new_owner_id, &new_account);
        self.refund_storage(initial_storage);
    }

    #[payable]
    pub fn transfer(&mut self, new_owner_id: AccountId, amount: U128) {
        // NOTE: New owner's Account ID checked in transfer_from.
        // Storage fees are also refunded in transfer_from.
        self.transfer_from(env::predecessor_account_id(), new_owner_id, amount);
    }

    pub fn get_allowance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> U128 {
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        self.get_account(&owner_id)
            .get_allowance(&escrow_account_id)
            .into()
    }

    /// Mints given amount to the smart contract caller
    #[allow(dead_code)]
    pub fn mint_to(&mut self, amount: u128, target: AccountId) -> U128 {
        let caller = env::predecessor_account_id();
        if caller != self.tokenizer {
            env::panic(b"Caller is not Tokenizer");
        }

        self.total_supply += amount;
        let mut account = self.get_account(&target);
        account.balance += amount;
        self.set_account(&target, &account);
        account.balance.into()
    }

    #[allow(dead_code)]
    pub fn burn_from(&mut self, amount: u128, target: AccountId) {
        let caller = env::predecessor_account_id();
        if caller != self.tokenizer {
            env::panic(b"Caller is not Tokenizer");
        }

        // Retrieving the account from the state.
        let mut account = self.get_account(&target);

        // Checking and updating unlocked balance
        if account.balance <= amount {
            env::panic(b"Not enough balance");
        }
        account.balance -= amount;

        // If transferring by escrow, need to check and update allowance.
        if caller != target {
            let allowance = account.get_allowance(&caller);
            if allowance < amount {
                env::panic(b"Not enough allowance");
            }
            account.set_allowance(&caller, allowance - amount);
        }

        self.total_supply -= amount;
        self.set_account(&target, &account);
    }
}

impl ScaleToken {
    fn get_account(&self, owner_id: &AccountId) -> Account {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        let account_hash = env::sha256(owner_id.as_bytes());
        self.accounts
            .get(&account_hash)
            .unwrap_or_else(|| Account::new(account_hash))
    }

    fn set_account(&mut self, owner_id: &AccountId, account: &Account) {
        let account_hash = env::sha256(owner_id.as_bytes());
        if account.balance > 0 || account.allowances_count > 0 {
            self.accounts.insert(&account_hash, &account);
        } else {
            self.accounts.remove(&account_hash);
        }
    }

    fn refund_storage(&self, initial_storage: StorageUsage) {
        let current_storage = env::storage_usage();
        let attached_deposit = env::attached_deposit();
        let refund_amount = if current_storage > initial_storage {
            let required_deposit =
                Balance::from(current_storage - initial_storage) * STORAGE_PRICE_PER_BYTE;
            assert!(
                required_deposit <= attached_deposit,
                "The required attached deposit is {}, but the given attached deposit is is {}",
                required_deposit,
                attached_deposit,
            );
            attached_deposit - required_deposit
        } else {
            attached_deposit
                + Balance::from(initial_storage - current_storage) * STORAGE_PRICE_PER_BYTE
        };
        if refund_amount > 0 {
            env::log(format!("Refunding {} tokens for storage", refund_amount).as_bytes());
            Promise::new(env::predecessor_account_id()).transfer(refund_amount);
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

    #[test]
    fn test_transfer_to_a_different_account_works() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

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

    #[test]
    #[should_panic(expected = "The new owner should be different from the current owner")]
    fn test_transfer_to_self_fails() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

        context.predecessor_account_id = carol();
        context.storage_usage = env::storage_usage();
        context.attached_deposit = 1000 * STORAGE_PRICE_PER_BYTE;
        testing_env!(context.clone());
        let transfer_amount = mint_balance / 3;
        contract.transfer(carol(), transfer_amount.into());
    }

    #[test]
    #[should_panic(expected = "Can not increment allowance for yourself")]
    fn test_increment_allowance_to_self_fails() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

        context.predecessor_account_id = carol();
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.approve(carol(), (mint_balance / 2).into());
    }

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

    #[test]
    #[should_panic(
        expected = "The required attached deposit is 12400000000000000000000, but the given attached deposit is is 0"
    )]
    fn test_increment_allowance_with_insufficient_attached_deposit() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

        context.predecessor_account_id = carol();
        context.attached_deposit = 0;
        testing_env!(context.clone());
        contract.approve(bob(), (mint_balance / 2).into());
    }

    #[test]
    fn test_carol_escrows_to_bob_transfers_to_alice() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

        context.predecessor_account_id = carol();
        context.storage_usage = env::storage_usage();

        context.is_view = true;
        testing_env!(context.clone());
        assert_eq!(contract.get_total_supply().0, mint_balance);

        let allowance = mint_balance / 3;
        let transfer_amount = allowance / 3;
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.approve(bob(), allowance.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);

        // Acting as bob now
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        contract.transfer_from(carol(), alice(), transfer_amount.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(
            contract.get_balance(carol()).0,
            mint_balance - transfer_amount
        );
        assert_eq!(contract.get_balance(alice()).0, transfer_amount);
        assert_eq!(
            contract.get_allowance(carol(), bob()).0,
            allowance - transfer_amount
        );
    }

    #[test]
    fn test_carol_escrows_to_bob_locks_and_transfers_to_alice() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

        context.predecessor_account_id = carol();

        context.storage_usage = env::storage_usage();

        context.is_view = true;
        testing_env!(context.clone());
        assert_eq!(contract.get_total_supply().0, mint_balance);

        let allowance = mint_balance / 3;
        let transfer_amount = allowance / 3;
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.approve(bob(), allowance.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(contract.get_allowance(carol(), bob()).0, allowance);
        assert_eq!(contract.get_balance(carol()).0, mint_balance);

        // Acting as bob now
        context.is_view = false;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        context.predecessor_account_id = bob();
        testing_env!(context.clone());
        contract.transfer_from(carol(), alice(), transfer_amount.into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();

        context.is_view = true;
        context.attached_deposit = 0;
        testing_env!(context.clone());
        assert_eq!(
            contract.get_balance(carol()).0,
            (mint_balance - transfer_amount)
        );
        assert_eq!(contract.get_balance(alice()).0, transfer_amount);
        assert_eq!(
            contract.get_allowance(carol(), bob()).0,
            allowance - transfer_amount
        );
    }

    #[test]
    fn test_self_allowance_set_for_refund() {
        let mut context = get_context(tokenizer());
        testing_env!(context.clone());
        let mint_balance = 1_000_000_000_000_000u128;
        let mut contract = ScaleToken::new(tokenizer());
        contract.mint_to(mint_balance, carol());

        context.predecessor_account_id = carol();
        context.storage_usage = env::storage_usage();

        let initial_balance = context.account_balance;
        let initial_storage = context.storage_usage;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.approve(bob(), (mint_balance / 2).into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();
        assert_eq!(
            context.account_balance,
            initial_balance
                + Balance::from(context.storage_usage - initial_storage) * STORAGE_PRICE_PER_BYTE
        );

        let initial_balance = context.account_balance;
        let initial_storage = context.storage_usage;
        testing_env!(context.clone());
        context.attached_deposit = 0;
        testing_env!(context.clone());
        contract.approve(bob(), (mint_balance / 2).into());
        context.storage_usage = env::storage_usage();
        context.account_balance = env::account_balance();
        assert!(context.storage_usage == initial_storage);
        assert!(context.account_balance == initial_balance);
        assert_eq!(
            context.account_balance,
            initial_balance
                - Balance::from(initial_storage - context.storage_usage) * STORAGE_PRICE_PER_BYTE
        );
    }
}
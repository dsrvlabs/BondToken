use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise, StorageUsage,
};
use uint::construct_uint;

/// 같은 디렉토리의 internal.rs 요구, 실질 함수들 많이 들어가있는 용도.
/// mod internal;
// fungible token interface
// mod nep21;
mod validator;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

/// Price per 1 byte of storage from mainnet genesis config.
const STORAGE_PRICE_PER_BYTE: Balance = 100000000000000000000;

pub type NumStakeShares = Balance;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

/// Near Public key 하나가 가지고 있는 정보
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenAccount {
    /// ERC20의 balance는 하나만 가지고 있음
    pub balance: Balance,
    /// ERC20의 allowances는 여럿 가지고 있을 수 있음.
    pub allowances: LookupMap<Vec<u8>, Balance>,
    /// allowances의 허용 숫자.
    pub num_allowances: u32,
}

impl TokenAccount {
    pub fn new(account_hash: Vec<u8>) -> Self {
        Self {
            balance: 0,
            allowances: LookupMap::new(account_hash),
            num_allowances: 0
        }
    }

    /// erc20의 approve,
    /// allowance 설정 다만 0이하라면 처리 없음.
    pub fn set_allowance(&mut self, escrow_account_id: &AccountId, allowance: Balance) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());

        if allowance > 0 {
            if self.allowances.insert(&escrow_hash, &allowance).is_none() {
                self.num_allowances += 1;
            }
        } else {
            if self.allowances.remove(&escrow_hash).is_some() {
                self.num_allowances -= 1;
            }
        }
    }

    pub fn get_allowance(&self, escrow_account_id: &AccountId) -> Balance {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        return self.allowances.get(&escrow_hash).unwrap_or(0);
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BondToken {
    /// sha256(AccountId) -> Account Detail
    pub accounts: LookupMap<Vec<u8>, TokenAccount>,

    /// 토큰이 제공하는 소수점자리
    /// 기본 18u8
    pub decimals: u8,

    /// 토큰 계약 소유자
    pub owner: AccountId,

    /// total token balance
    pub total_supply: Balance,

    /// 토큰 수량을 계산하는 scale factor
    pub scale_factor: Balance,
}

/// 초기화 되기 전에 사용되는 것을 방지
impl Default for BondToken {
    fn default() -> Self {
        panic!("Fun token should be initialized before usage")
    }
}

#[near_bindgen]
impl BondToken {
    
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        // state가 있는지 확인.
        assert!(!env::state_exists(), "Already initialized");
        let total_supply = total_supply.into();
        let mut bt = Self {
            accounts: LookupMap::new(b"a".to_vec()),
            decimals: 18u8,
            owner: env::predecessor_account_id(),
            total_supply,
            scale_factor: 1u128 };
        let mut account = bt.get_account(&owner_id);
        account.balance = total_supply;
        bt.set_account(&owner_id, &account);
        bt
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

    #[payable]
    pub fn update_scale_factor(&mut self, scale: U128) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Not Owner");
        self.scale_factor = scale.into();
    }

    #[payable]
    pub fn mint(&mut self, amount: U128) {
        let initial_storage = env::storage_usage();
        assert_eq!(env::predecessor_account_id(), self.owner, "Not Owner");
        let owner_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        let mut account = self.get_account(&self.owner);
        account.balance += amount;
        self.set_account(&owner_id, &account);
        self.refund_storage(initial_storage);
    }

    pub fn get_total_supply(&self) -> U128 {
        self.total_supply.into()
    }

    pub fn get_balance(&self, owner_id: AccountId) -> U128 {
        let account = self.get_account(&owner_id);
        (account.balance * self.scale_factor).into()
    }

    pub fn get_allowance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> U128 {
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        self.get_account(&owner_id).get_allowance(&escrow_account_id).into()
    }
}

/// Helper Function set.
impl BondToken {
    /// Helper method to get the account details for `owner_id`.
    fn get_account(&self, owner_id: &AccountId) -> TokenAccount {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        let account_hash = env::sha256(owner_id.as_bytes());
        self.accounts.get(&account_hash).unwrap_or_else(|| TokenAccount::new(account_hash))
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_account(&mut self, owner_id: &AccountId, account: &TokenAccount) {
        let account_hash = env::sha256(owner_id.as_bytes());
        if account.balance > 0 || account.num_allowances > 0 {
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
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise, PromiseOrValue, PromiseResult
};

/**
 * external contract interface load.
 */ 
use token::{nep21};
use registry::{registry};
use crate::utils::{ is_promise_success };
mod validator;


#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

const DEPOSIT_AND_STAKING_GAS: u64 = 100_000_000_000_000;
const STORAGE_PRICE_PER_BYTE: Balance = 100_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokenizer {
    token: AccountId,
    governance: AccountId,
    registry: AccountId,
}

impl Default for Tokenizer {
    fn default() -> Self {
        panic!("Tokenizer should be initialized before usage")
    }
}

#[near_bindgen]
impl Tokenizer {

    #[init]
    pub fn new(token: AccountId, governance: AccountId, registry: AccountId) -> Self {
        assert!(!env::state_exists(), "Registry: Already initialized");
        Self {
            token,
            governance,
            registry
        }
    }

    // #[payable]
    pub fn deposit(&self) {
        let owner_id = env::predecessor_account_id();
        let amount = env::attached_deposit();

        self.validators();

        let vector = env::promise_result(0);

        
    }

    fn deposit_and_stake(&mut self, validator: AccountId, amount: Balance) {
        validator::ext_validator::deposit_and_stake(&validator, amount, DEPOSIT_AND_STAKING_GAS);
        env::log(format!("{} to staking amount {}", validator, amount).as_bytes());
    }

    #[result_serializer(borsh)]
    fn validators(&self) -> Promise {
        registry::ext_registry::get_validators(&self.registry, 0, DEPOSIT_AND_STAKING_GAS)
    }
}

use crate::*;
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId
};

#[near_bindgen]
impl ScaleToken {
    pub fn decimals(&self) -> u8 {
        return self.decimals;
    }

    /// Returns total supply of tokens.
    pub fn get_total_supply(&self) -> U128 {
        ((self.total_credit / self.scale_factor) * 10u128.pow(24)).into()
    }

    /// Returns balance of the `owner_id` account.
    pub fn get_balance(&self, owner_id: AccountId) -> U128 {
        ((self.get_account(&owner_id).credit / self.scale_factor) * 10u128.pow(24)).into()
    }

    pub fn get_credit(&self, owner_id: AccountId) -> U128 {
        self.get_account(&owner_id).credit.into()
    }

    /// Increments the `allowance` for `escrow_account_id` by `amount` on the account of the caller of this contract
    /// (`predecessor_id`) who is the balance owner.
    /// Requirements:
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    #[payable]
    pub fn inc_allowance(&mut self, escrow_account_id: AccountId, amount: U128) {
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
        let current_allowance = account.get_allowance(&escrow_account_id);
        account.set_allowance(
            &escrow_account_id,
            current_allowance.saturating_add(amount.0),
        );
        self.set_account(&owner_id, &account);
        self.refund_storage(initial_storage);
    }

    /// Decrements the `allowance` for `escrow_account_id` by `amount` on the account of the caller of this contract
    /// (`predecessor_id`) who is the balance owner.
    /// Requirements:
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    #[payable]
    pub fn dec_allowance(&mut self, escrow_account_id: AccountId, amount: U128) {
        let initial_storage = env::storage_usage();
        assert!(
            env::is_valid_account_id(escrow_account_id.as_bytes()),
            "Escrow account ID is invalid"
        );
        let owner_id = env::predecessor_account_id();
        if escrow_account_id == owner_id {
            env::panic(b"Can not decrement allowance for yourself");
        }
        let mut account = self.get_account(&owner_id);
        let current_allowance = account.get_allowance(&escrow_account_id);
        account.set_allowance(
            &escrow_account_id,
            current_allowance.saturating_sub(amount.0),
        );
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
        if amount.0 == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
        let credit_amount = amount.0 / self.scale_factor;

        assert_ne!(
            owner_id, new_owner_id,
            "The new owner should be different from the current owner"
        );
        // Retrieving the account from the state.
        let mut account = self.get_account(&owner_id);

        env::log(format!("transfer_from {} to {}, {}", owner_id, new_owner_id, amount.0).as_bytes());

        // Checking and updating unlocked balance
        if account.credit < credit_amount {
            env::panic(format!("Not enough balance {} {} {} {}", account.credit, credit_amount, amount.0, self.scale_factor).as_bytes());
        }
        account.credit -= credit_amount;

        // If transferring by escrow, need to check and update allowance.
        let escrow_account_id = env::predecessor_account_id();
        if escrow_account_id != owner_id {
            let allowance = account.get_allowance(&escrow_account_id);
            if allowance < amount.0 {
                env::panic(b"Not enough allowance");
            }
            account.set_allowance(&escrow_account_id, allowance - amount.0);
        }

        // Saving the account back to the state.
        self.set_account(&owner_id, &account);

        // Deposit amount to the new owner and save the new account to the state.
        let mut new_account = self.get_account(&new_owner_id);
        new_account.credit += credit_amount;
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
    pub fn mint_to(&mut self, amount: Balance, target: AccountId) {
        self.assert_tokenizer();

        let mut account = self.get_account(&target);
        let credit_amount = amount * self.scale_factor;

        self.total_credit += credit_amount;
        account.credit += credit_amount;

        env::log(format!("transfer_from {} to {}, {}", "0", target, amount).as_bytes());

        self.set_account(&target, &account);
    }

    #[allow(dead_code)]
    pub fn burn_from(&mut self, amount: Balance, target: AccountId) {
        self.assert_tokenizer();

        // Retrieving the account from the state.
        let mut account = self.get_account(&target);
        let credit_amount = amount * self.scale_factor;

        // Checking and updating unlocked balance
        if account.credit <= credit_amount {
            env::panic(b"Not enough balance");
        }
        account.credit -= credit_amount;

        // If transferring by escrow, need to check and update allowance.
        if env::predecessor_account_id() != target {
            let allowance = account.get_allowance(&env::predecessor_account_id());
            if allowance < amount {
                env::panic(b"Not enough allowance");
            }
            account.set_allowance(&env::predecessor_account_id(), allowance - amount);
        }

        env::log(format!("transfer_from {} to {}, {}", target, "0", amount).as_bytes());

        self.total_credit -= credit_amount;
        self.set_account(&target, &account);
    }
}

use crate::*;
use near_sdk::{near_bindgen, AccountId, Balance};

/********************/
/* Internal methods */
/********************/

impl ScaleToken {
    pub(crate) fn assert_tokenizer(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.tokenizer,
            "ScaleToken: Caller is not Tokenizer"
        )
    }

    pub(crate) fn get_account(&self, owner_id: &AccountId) -> Account {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        let account_hash = env::sha256(owner_id.as_bytes());
        self.accounts
            .get(&account_hash)
            .unwrap_or_else(|| Account::new(account_hash))
    }

    pub(crate) fn set_account(&mut self, owner_id: &AccountId, account: &Account) {
        let account_hash = env::sha256(owner_id.as_bytes());
        if account.credit > 0 || account.allowances_count > 0 {
            self.accounts.insert(&account_hash, &account);
        } else {
            self.accounts.remove(&account_hash);
        }
    }

    pub(crate) fn refund_storage(&self, initial_storage: StorageUsage) {
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

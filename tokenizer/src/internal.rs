use crate::*;
use near_sdk::{near_bindgen, AccountId};

/********************/
/* Internal methods */
/********************/

#[near_bindgen]
impl Tokenizer {
    fn assert_governance(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.governance,
            "Registry: Caller is not Governance"
        )
    }

    fn get_withdraw(&self, owner_id: &AccountId) -> WithdrawAccount {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        let account_hash = env::sha256(owner_id.as_bytes());
        self.withdraws.get(&account_hash).unwrap_or_else(|| WithdrawAccount::new())
    }

    fn set_withdraw(&mut self, owner_id: &AccountId, withdraw: &WithdrawAccount) {
        let account_hash = env::sha256(owner_id.as_bytes());
        if withdraw.claim_balance > 0 {
            self.withdraws.insert(&account_hash, &withdraw);
        } else {
            self.withdraws.remove(&account_hash);
        }
    }
}
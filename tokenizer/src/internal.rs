use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey};

/********************/
/* Internal methods */
/********************/

#[near_bindgen]
impl Tokenizer {
    pub fn assert_governance(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.governance,
            "Registry: Caller is not Governance"
        )
    }
}
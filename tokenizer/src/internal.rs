use crate::*;
use near_sdk::{near_bindgen, AccountId, Balance};

/********************/
/* Internal methods */
/********************/

#[near_bindgen]
impl Tokenizer {
    /// Governance만 수행 가능
    fn assert_governance(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.governance,
            "Registry: Caller is not Governance"
        )
    }

    /// Near 출금정보 가져오기
    fn get_withdraw(&self, owner_id: &AccountId) -> WithdrawAccount {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        let account_hash = env::sha256(owner_id.as_bytes());
        self.withdraws.get(&account_hash).unwrap_or_else(|| WithdrawAccount::new())
    }

    /// Near 출금정보 설정
    fn set_withdraw(&mut self, owner_id: &AccountId, withdraw: &WithdrawAccount) {
        let account_hash = env::sha256(owner_id.as_bytes());
        if withdraw.claim_balance > 0 {
            self.withdraws.insert(&account_hash, &withdraw);
        } else {
            self.withdraws.remove(&account_hash);
        }
    }

    /// 모든 검증인으로 부터 비율 맞게 unstake
    fn undelegate(&self, amount: Balance) {
        let mut total_ratio;
        for tuple in self.registry.get_validators().iter() {
            total_ratio += tuple.1;
        }

        for tuple in self.registry.get_validators().iter() {
            let bal = amount * (tuple.1.into() / total_ratio.into());
            ext_validator::unstake(bal, &tuple.0, NO_DEPOSIT, SINGLE_CALL_GAS);
            env::log(
                format!(
                    "unstake amount @{} from validator @{}",
                    bal.into(),
                    tuple.0
                )
                .as_bytes()
            );
        }
    }
}
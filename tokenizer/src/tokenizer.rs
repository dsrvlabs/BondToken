use crate::*;
use near_sdk::{near_bindgen, AccountId, Promise, PublicKey, Balance};

#[near_bindgen]
impl Tokenizer {
    #[payable]
    pub fn deposit(&self) {
        let owner_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        let mut total_ratio;
        for tuple in self.registry.get_validators().iter() {
            total_ratio += tuple.1;
        }

        for tuple in self.registry.get_validators().iter() {
            let mut amount * (tuple.1.into() / total_ratio);
            ext_validator::deposit_and_stake(&tuple.0, amount, SINGLE_CALL_GAS);
        }
    }

    /// yNear를 소각하고, withdraw에 대기 토큰 등록
    pub fn burn(&mut self, amount: Balance) {
        let owner_id = env::predecessor_account_id();

        // 이 단계에서 토큰이 소각되지 않으면 withdraw 등록되면 안됨
        // using promise?
        ext_ft::burn_from(amount, owner_id, &self.token, NO_DEPOSIT, SINGLE_CALL_GAS);

        // undelegate near with ratio
        self.undelegate(amount);

        // withdraw에 등록
        let mut withdraw = self.get_withdraw(owner_id);
        withdraw.claim_balance += amount;
        self.set_withdraw(&owner_id, &withdraw);
        
        self.total_waiting += amount;
    }

    // @TODO: burn 할 때 언스테이크 호출 함수
    fn undelegate(&self, amount: Balance) {
        let mut total_ratio;
        for tuple in self.registry.get_validators().iter() {
            total_ratio += tuple.1;
        }

        for tuple in self.registry.get_validators().iter() {
            let mut amount * (tuple.1.into() / total_ratio);
            ext_validator::unstake(amount, &tuple.0, NO_DEPOSIT, SINGLE_CALL_GAS);
        }
    }

    // @TODO: burn 이후에 대기 상태에 있는 Near를 출금할 수 있도록 하는 함수
    pub fn withdraw(&self) {
        let owner_id = env::predecessor_account_id();
        let mut withdraw = self.get_withdraw(owner_id);

        withdraw.claim_balance

        Promise::new(owner_id).transfer(withdraw.claim_balance)
    }
}
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

        //@TODO: deposit balance - predefined gas balance
        for tuple in self.registry.get_validators().iter() {
            let bal = deposit * (tuple.1.into() / total_ratio);
            ext_validator::deposit_and_stake(&tuple.0, amount, SINGLE_CALL_GAS);
        }

        ext_ft::mint_to(amount, owner_id, &self.token, NO_DEPOSIT, SINGLE_CALL_GAS);
    }

    /// yNear를 소각하고, withdraw에 대기 토큰 등록
    pub fn burn(&mut self, amount: Balance) {
        let owner_id = env::predecessor_account_id();

        // 이 단계에서 토큰이 소각되지 않으면 withdraw 등록되면 안됨
        // using promise?
        ext_ft::burn_from(amount, owner_id, &self.token, NO_DEPOSIT, SINGLE_CALL_GAS);

        // undelegate near
        self.undelegate(amount);

        // withdraw에 등록
        let mut withdraw = self.get_withdraw(owner_id);
        withdraw.claim_balance += amount;
        self.set_withdraw(&owner_id, &withdraw);
        
        self.total_waiting += amount;
    }

    pub fn withdraw(&mut self) {
        let owner_id = env::predecessor_account_id();
        let mut withdraw = self.get_withdraw(owner_id);

        if env::epoch_height() < withdraw.unstaked_available_epoch_height {
            env::panic(b"tokenizer/not withdrawable");
        }
        self.total_waiting -= withdraw.claim_balance;
        withdraw.claim_balance -= withdraw.claim_balance;
        Promise::new(owner_id).transfer(withdraw.claim_balance);
    }

    pub fn total_staked(&self) -> Balance {
        let mut total;
        for tuple in self.registry.get_validators().iter() {
            total += env::validator_stake(&tuple.0);
        }
    }
}
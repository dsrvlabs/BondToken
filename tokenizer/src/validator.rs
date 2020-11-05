use near_sdk::ext_contract;
use near_sdk::json_types::U128;

#[ext_contract(ext_validator)]
pub trait ext_validator {
    /// Deposits the attached amount into the inner account of the predecessor.
    // #[payable]
    fn deposit(&mut self);

    /// Deposits the attached amount into the inner account of the predecessor and stakes it.
    // #[payable]
    fn deposit_and_stake(&mut self);

    /// Withdraws the entire unstaked balance from the predecessor account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    fn withdraw_all(&mut self);

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    fn withdraw(&mut self, amount: U128);

    /// Stakes all available unstaked balance from the inner account of the predecessor.
    fn stake_all(&mut self);

    /// Stakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough unstaked balance.
    fn stake(&mut self, amount: U128);

    /// Unstakes all staked balance from the inner account of the predecessor.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    fn unstake_all(&mut self);

    /// Unstakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough staked balance.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    fn unstake(&mut self, amount: U128);
}
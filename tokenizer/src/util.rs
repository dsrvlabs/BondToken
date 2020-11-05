use near_sdk::{env, PromiseResult};

// Near supported 1e24
pub const DECIMAL: u128 = 1_000_000_000_000_000_000_000_000;

pub fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}
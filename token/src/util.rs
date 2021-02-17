use crate::*;

use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

// Near supported 1e24
pub const DECIMAL: u128 = 1_000_000_000_000_000_000_000_000;
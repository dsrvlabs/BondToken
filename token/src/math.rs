use crate::*;
use near_sdk::json_types::{U128};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

/********************/
/* Internal methods */
/********************/

impl ScaleToken {
    pub(crate) fn multiply_scale(x: U128, y: U128) -> U128 {
        ((U256::from(x.0) * U256::from(y.0)) / U256::from(10u128.pow(24))).as_u128().into()
    }

    pub(crate) fn devide_scale(x: U128, y: U128) -> U128 {
        let min = std::cmp::min(x.0, y.0);
        let max = std::cmp::max(x.0, y.0);
        ((U256::from(max) / U256::from(min)) * U256::from(10u128.pow(24))).as_u128().into()
    }
}
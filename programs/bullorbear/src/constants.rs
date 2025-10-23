// TODO: Use the pubkeys from config
pub const ADMIN_PUBKEY_STR: &str = "FiM1NVpzFWih7s8ff99T1tHiFXLLFGRb5Bd7QxgJkPFm";
pub const OPERATOR_PUBKEY_STR: &str = "FiM1NVpzFWih7s8ff99T1tHiFXLLFGRb5Bd7QxgJkPFm";

pub const MAXIMUM_AGE: u64 = 300; // 5 minutes TODO: consider removing this? as even an old price can be retrieved from Pyth
pub const FEED_ID: &str = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43"; // TODO: make it configurable?
pub const BETTING_CUTOFF: u8 = 3; // prevent user from submitting bets X seconds before round starts
pub const MAX_TREASURY_FEE: u16 = 1000; // 10%

#[macro_export]
macro_rules! require_admin_or_operator {
    ($signer:expr) => {{
        use std::str::FromStr;
        let admin_pubkey = Pubkey::from_str($crate::constants::ADMIN_PUBKEY_STR).unwrap();
        let operator_pubkey = Pubkey::from_str($crate::constants::OPERATOR_PUBKEY_STR).unwrap();
        require!(
            $signer == admin_pubkey || $signer == operator_pubkey,
            $crate::errors::ErrorCode::UnauthorizedOperator
        );
    }};
}

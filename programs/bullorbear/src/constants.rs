pub const MAXIMUM_AGE: u64 = 1800; // 30 minute
pub const FEED_ID: &str = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43"; // Pyth BTC/USD feed on Solana
/// Betting cutoff before round lock (in seconds)
///
/// DESIGN DECISION: This is intentionally set to a short 3-second window.
/// This is NOT a security vulnerability but a game design feature that provides:
/// - Transparency: Users can see live pool amounts and positions
/// - Excitement: Last-second betting adds game dynamics
/// - Fairness: All users have equal access to pool information
///
/// The lock price is determined by oracle at lock_ts, not by user bets,
/// so knowing pool distribution doesn't guarantee winning. Users betting
/// late take the risk that the lock transaction might process before their bet.
///
/// This is similar to traditional prediction markets where order book depth
/// is visible to all participants.
pub const BETTING_CUTOFF: u8 = 3;
pub const MAX_TREASURY_FEE: u16 = 1000; // 10%
pub const SECONDS_BEFORE_REFUND_AVAILABLE: i64 = 1800; // 30 minutes

#[macro_export]
macro_rules! require_admin_or_operator {
    ($signer:expr, $config:expr) => {{
        require!(
            $signer == $config.admin || $signer == $config.operator,
            $crate::errors::ErrorCode::UnauthorizedOperator
        );
    }};
}

#[macro_export]
macro_rules! require_admin {
    ($signer:expr, $config:expr) => {{
        require!(
            $signer == $config.admin,
            $crate::errors::ErrorCode::UnauthorizedOperator
        );
    }};
}

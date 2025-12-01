use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Can run genesis_lock_round only once")]
    GenesisLockRoundOnce,
    #[msg("Invalid round epoch")]
    InvalidEpoch,
    #[msg("Round not bettable")]
    RoundNotBettable,
    #[msg("Bet amount below minimum")]
    BetTooSmall,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Round not ended")]
    RoundNotEnded,
    #[msg("Unauthorized operator")]
    UnauthorizedOperator,
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    #[msg("Invalid fee percentage")]
    InvalidFee,
    #[msg("The price feed account is invalid or not BTC/USD.")]
    InvalidPriceFeed,
    #[msg("The price feed is stale or not currently trading.")]
    StalePrice,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Betting is closed for this round")]
    BettingClosed,
    #[msg("Unresolved bets exist for this round")]
    UnresolvedBetsExist,
    #[msg("Insufficient treasury funds")]
    InsufficientTreasuryFunds,
    #[msg("Game is paused")]
    GamePaused,
    #[msg("Refund not yet available")]
    RefundNotYetAvailable,
    #[msg("Price timestamp not within 1 second window")]
    PriceTimestampMismatch,
    #[msg("Invalid admin or operator configuration")]
    InvalidAdminOrOperator,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Invalid account size")]
    InvalidAccountSize,
    #[msg("Insufficient account data")]
    InsufficientAccountData,
    #[msg("Bet serialization failed")]
    BetSerializationFailed,
    #[msg("Bet verification failed")]
    BetVerificationFailed,
    #[msg("Invalid discriminator")]
    InvalidDiscriminator,
}

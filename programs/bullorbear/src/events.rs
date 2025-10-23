use crate::state::Position;
use anchor_lang::prelude::*;

#[event]
pub struct GenesisInitialized {
    pub interval: u16,
    pub epoch: u64,
    pub lock_ts: i64,
}

#[event]
pub struct RoundInitialized {
    pub epoch: u64,
    pub start_ts: i64,
    pub lock_ts: i64,
    pub close_ts: i64,
    pub key: Pubkey,
}

#[event]
pub struct RoundStarted {
    pub key: Pubkey,
    pub epoch: u64,
    pub close_ts: i64,
    pub lock_price: i64,
    pub lock_price_exponent: i32,
}

#[event]
pub struct RoundClosed {
    pub key: Pubkey,
    pub epoch: u64,
    pub close_ts: i64,
    pub close_price: i64,
    pub close_price_exponent: i32,
}

#[event]
pub struct Pause {
    pub epoch: u64,
}

#[event]
pub struct Unpause {
    pub epoch: u64,
}

#[event]
pub struct BetEvent {
    pub epoch: u64,
    pub position: Position,
    pub bet_amount: u64,
    pub total_amount: u64,
    pub bull_amount: u64,
    pub bear_amount: u64,
    pub bull_total_bets: u32,
    pub bear_total_bets: u32,
    pub user: Pubkey,
}

#[event]
pub struct RewardsCalculated {
    pub epoch: u64,
    pub reward_base: u64,
    pub reward_amount: u64,
    pub treasury_amount: u64,
}

#[event]
pub struct Claim {
    pub user: Pubkey,
    pub epoch: u64,
    pub amount: u64,
    pub winning_position: Position,
    pub payout_ratio: f64,
}

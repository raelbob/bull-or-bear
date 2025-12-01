use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub locked_once: bool,
    pub paused: bool,
    pub interval_seconds: u16,
    pub min_bet_amount: u64,
    pub treasury_fee: u16,
    pub treasury_amount: u64,
    pub pending_bet_amount: u64,
    pub current_epoch: u64,
    pub last_available_epoch: u64,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Round {
    pub epoch: u64,
    pub start_ts: i64,
    pub lock_ts: i64,
    pub close_ts: i64,
    pub lock_price: Option<i64>,
    pub lock_price_exponent: Option<i32>,
    pub close_price: Option<i64>,
    pub close_price_exponent: Option<i32>,
    pub total_amount: u64,
    pub bull_amount: u64,
    pub bear_amount: u64,
    pub bull_total_bets: u32,
    pub bear_total_bets: u32,
    pub reward_base: u64,
    pub reward_amount: u64,
    pub unresolved_bets_count: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum Position {
    Bull,
    Bear,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Bet {
    pub user: Pubkey,
    pub epoch: u64,
    pub position: Position,
    pub amount: u64,
}

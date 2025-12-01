use crate::constants::{FEED_ID, MAXIMUM_AGE};
use crate::contexts::CloseBetContext;
use crate::errors::ErrorCode;
use crate::events::{BetEvent, RewardsCalculated, RoundInitialized};
use crate::state::{Bet, Config, Position, Round};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, Price, PriceUpdateV2};

pub fn require_not_paused(config: &Account<Config>) -> Result<()> {
    require!(!config.paused, ErrorCode::GamePaused);
    Ok(())
}

pub fn get_next_full_minute() -> Result<i64> {
    let now = Clock::get()?.unix_timestamp;
    Ok(((now + 59) / 60) * 60)
}

pub fn initialize_round(
    round: &mut Account<Round>,
    epoch: u64,
    interval_seconds: u16,
    start_ts: i64,
) -> Result<()> {
    round.epoch = epoch;
    round.start_ts = start_ts;
    round.lock_ts = start_ts + interval_seconds as i64;
    round.close_ts = round.lock_ts + interval_seconds as i64;

    emit!(RoundInitialized {
        epoch: round.epoch,
        start_ts: round.start_ts,
        lock_ts: round.lock_ts,
        close_ts: round.close_ts,
        key: round.key(),
    });

    Ok(())
}

pub fn get_price(price_update: &Account<PriceUpdateV2>, expected_timestamp: i64) -> Result<Price> {
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(FEED_ID)?,
    )?;

    // SECURITY NOTE: This exact timestamp match is REQUIRED for provable fairness.
    // This is NOT a vulnerability because:
    // 1. Pyth publishes exactly ONE price per timestamp - there are no multiple prices to choose from
    // 2. The operator cannot create fake PriceUpdateV2 accounts (only Pyth can)
    // 3. The exact match ensures deterministic outcomes that users can verify
    // 4. Any tolerance window would actually REDUCE security by allowing price selection
    //
    // The game's fairness depends on using the EXACT price at the round's lock/close timestamp.
    // This makes outcomes verifiable and prevents any manipulation.
    require!(
        price.publish_time == expected_timestamp,
        ErrorCode::PriceTimestampMismatch
    );

    Ok(price)
}

pub fn update_round_and_bet(
    round: &mut Account<Round>,
    bet: &mut Bet,
    user: &mut Signer,
    position: Position,
    amount: u64,
    config: &mut Account<Config>,
) -> Result<()> {
    round.total_amount = round
        .total_amount
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    round.unresolved_bets_count = round
        .unresolved_bets_count
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    // Update position-specific amounts and counts
    match position {
        Position::Bull => {
            round.bull_amount = round
                .bull_amount
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
            round.bull_total_bets = round
                .bull_total_bets
                .checked_add(1)
                .ok_or(ErrorCode::Overflow)?;
        }
        Position::Bear => {
            round.bear_amount = round
                .bear_amount
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
            round.bear_total_bets = round
                .bear_total_bets
                .checked_add(1)
                .ok_or(ErrorCode::Overflow)?;
        }
    }

    // Track funds in treasury immediately
    config.treasury_amount = config
        .treasury_amount
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    // Track pending bet amount (locked funds)
    config.pending_bet_amount = config
        .pending_bet_amount
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    bet.position = position;
    bet.user = user.key();
    bet.epoch = round.epoch;
    bet.amount = amount;

    emit!(BetEvent {
        epoch: bet.epoch,
        position: bet.position,
        bet_amount: bet.amount,
        user: bet.user,
        bull_amount: round.bull_amount,
        bear_amount: round.bear_amount,
        total_amount: round.total_amount,
        bull_total_bets: round.bull_total_bets,
        bear_total_bets: round.bear_total_bets,
    });

    Ok(())
}

pub fn calculate_rewards(round: &mut Account<Round>, config: &mut Account<Config>) -> Result<()> {
    let close_price = round.close_price.ok_or(ErrorCode::RoundNotEnded)?;
    let lock_price = round.lock_price.ok_or(ErrorCode::RoundNotEnded)?;

    let bull_won = close_price > lock_price;
    let bear_won = close_price < lock_price;

    if bull_won || bear_won {
        let (winner_pool, loser_pool) = if bull_won {
            (round.bull_amount, round.bear_amount)
        } else {
            (round.bear_amount, round.bull_amount)
        };

        // base bet of winners
        round.reward_base = winner_pool;

        // Try standard fee on total
        let standard_treasury = round
            .total_amount
            .checked_mul(config.treasury_fee as u64)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(10_000)
            .ok_or(ErrorCode::MathOverflow)?;

        let standard_reward = round.total_amount - standard_treasury;

        // If winners would lose money → charge fee only on losers
        if standard_reward < round.reward_base {
            // Fee only on losing pool
            let treasury = loser_pool
                .checked_mul(config.treasury_fee as u64)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(10_000)
                .ok_or(ErrorCode::MathOverflow)?;

            round.reward_amount = round.total_amount - treasury;
            // Treasury fee is kept (already in treasury_amount), losers funds distributed to winners
        } else {
            // Standard mode
            round.reward_amount = standard_reward;
            // Treasury fee is kept (already in treasury_amount)
        }
        // NOTE: We intentionally DO NOT update pending_bet_amount here.
        // pending_bet_amount tracks unclaimed ORIGINAL bets, not reward payouts.
        // It gets decreased in close_bet() when users claim.
    } else {
        // Tie → house wins everything (already in treasury_amount)
        round.reward_base = 0;
        round.reward_amount = 0;

        // NOTE: pending_bet_amount stays unchanged - tracks original bets
        // until users claim via close_bet()
    }

    emit!(RewardsCalculated {
        epoch: round.epoch,
        reward_base: round.reward_base,
        reward_amount: round.reward_amount,
        treasury_amount: config.treasury_amount,
    });

    Ok(())
}

pub fn validate_claim(ctx: &Context<CloseBetContext>) -> Result<()> {
    let round = &ctx.accounts.round;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp > round.close_ts,
        ErrorCode::RoundNotEnded
    );
    require!(round.close_price.is_some(), ErrorCode::RoundNotEnded);

    Ok(())
}

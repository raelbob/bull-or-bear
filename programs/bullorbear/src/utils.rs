use crate::constants::{FEED_ID, MAXIMUM_AGE};
use crate::errors::ErrorCode;
use crate::events::{BetEvent, RewardsCalculated, RoundInitialized};
use crate::state::{Bet, Config, Position, Round};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, Price, PriceUpdateV2};

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

pub fn get_price(price_update: &Account<PriceUpdateV2>) -> Result<Price> {
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(FEED_ID)?,
    )?;

    Ok(price)
}

pub fn update_round_and_bet(
    round: &mut Account<Round>,
    bet: &mut Bet,
    user: &mut Signer,
    position: Position,
    amount: u64,
) -> Result<()> {
    round.total_amount = round
        .total_amount
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    round.unresolved_bets_count += 1;
    match position {
        Position::Bull => {
            round.bull_amount = round
                .bull_amount
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
            round.bull_total_bets += 1;
        }
        Position::Bear => {
            round.bear_amount = round
                .bear_amount
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
            round.bear_total_bets += 1;
        }
    }

    bet.amount = amount;
    bet.position = position;
    bet.user = user.key();
    bet.epoch = round.epoch;

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
    let mut reward_base: u64 = 0;
    let mut treasury_amount: u64 = 0;
    let mut reward_amount: u64 = 0;

    if round.close_price > round.lock_price {
        reward_base = round.bull_amount;
        treasury_amount = round
            .total_amount
            .checked_mul(config.treasury_fee as u64)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(10_000)
            .ok_or(ErrorCode::MathOverflow)?;
        reward_amount = round.total_amount - treasury_amount;
    } else if round.close_price < round.lock_price {
        reward_base = round.bear_amount;
        treasury_amount = round
            .total_amount
            .checked_mul(config.treasury_fee as u64)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(10_000)
            .ok_or(ErrorCode::MathOverflow)?;
        reward_amount = round.total_amount - treasury_amount;
    } else if round.close_price == round.lock_price {
        // House wins, all goes to treasury
        reward_base = 0;
        reward_amount = 0;
        treasury_amount = round.total_amount;
    }

    round.reward_base = reward_base;
    round.reward_amount = reward_amount;

    config.treasury_amount = config.treasury_amount.checked_add(treasury_amount).unwrap();

    emit!(RewardsCalculated {
        epoch: round.epoch,
        reward_base,
        reward_amount,
        treasury_amount,
    });

    Ok(())
}

pub fn validate_claim(ctx: &Context<crate::contexts::CloseBetContext>) -> Result<()> {
    let bet = &ctx.accounts.bet;
    let round = &ctx.accounts.round;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp > round.close_ts,
        ErrorCode::RoundNotEnded
    );
    require!(!bet.claimed, ErrorCode::AlreadyClaimed);
    require!(round.close_price != 0, ErrorCode::RoundNotEnded);

    Ok(())
}

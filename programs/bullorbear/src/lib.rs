#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::Discriminator;

declare_id!("F4Cu5nYYQYJU9qdqyDcZsMbadcNeADDZTqD9AnN12DFK");

#[macro_use]
pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod state;
pub mod utils;

use constants::*;
use contexts::*;
use errors::ErrorCode;
use events::*;
use state::*;
use utils::*;

#[program]
pub mod bullorbear {

    use super::*;

    pub const MAX_TREASURY_FEE: u16 = 1000; // 10%

    pub fn initialize(ctx: Context<GenesisInitialize>, interval: u16, _epoch: u64) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());
        // TODO: check if initialized is false
        let config = &mut ctx.accounts.config;
        config.interval_seconds = interval;
        config.min_bet_amount = 1_000_000_00;
        config.treasury_fee = 500; // 5%
        config.genesis_lock_once = false;
        config.genesis_initialized = true;
        config.current_epoch = match config.last_available_epoch {
            0 => 0,
            n => n + 1,
        };

        // Get the current timestamp
        let now = Clock::get()?.unix_timestamp;
        // Compute the next full minute (in seconds)
        let next_full_minute = ((now + 59) / 60) * 60;

        let round = &mut ctx.accounts.round;
        round.epoch = config.current_epoch;
        round.start_ts = next_full_minute;
        round.lock_ts = next_full_minute + config.interval_seconds as i64;
        round.close_ts = next_full_minute + (config.interval_seconds as i64 * 2);

        emit!(GenesisInitialized {
            epoch: config.current_epoch,
            interval: config.interval_seconds,
            lock_ts: round.lock_ts,
        });

        emit!(RoundInitialized {
            key: round.key(),
            epoch: round.epoch,
            start_ts: round.start_ts,
            lock_ts: round.lock_ts,
            close_ts: round.close_ts,
        });

        let next_round = &mut ctx.accounts.next_round;
        next_round.epoch = config.current_epoch + 1;
        next_round.start_ts = round.lock_ts;
        next_round.lock_ts = round.close_ts;
        next_round.close_ts = round.close_ts + config.interval_seconds as i64;

        emit!(RoundInitialized {
            epoch: next_round.epoch,
            start_ts: next_round.start_ts,
            lock_ts: next_round.lock_ts,
            close_ts: next_round.close_ts,
            key: next_round.key(),
        });

        config.last_available_epoch = next_round.epoch;

        Ok(())
    }

    pub fn config_initialize(
        ctx: Context<ConfigInitialize>,
        interval: u16,
        min_bet_amount: u64,
        treasury_fee: u16,
    ) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());

        let config = &mut ctx.accounts.config;
        config.genesis_lock_once = false;
        config.genesis_initialized = false;
        config.interval_seconds = interval;
        config.min_bet_amount = min_bet_amount;
        config.treasury_fee = treasury_fee;
        Ok(())
    }

    // TODO: remove this function at later stage, only for development purposes
    pub fn close_config(ctx: Context<CloseConfig>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.destination.key());

        let config = &mut ctx.accounts.config;
        let destination = &mut ctx.accounts.destination;

        **destination.lamports.borrow_mut() += **config.to_account_info().lamports.borrow();
        **config.to_account_info().lamports.borrow_mut() = 0;

        Ok(())
    }

    pub fn close_round(ctx: Context<CloseRoundContext>, _epoch: u64) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.destination.key());

        let round = &mut ctx.accounts.round;
        let destination = &mut ctx.accounts.destination;

        require_eq!(
            round.unresolved_bets_count,
            0,
            ErrorCode::UnresolvedBetsExist
        );

        **destination.lamports.borrow_mut() += **round.to_account_info().lamports.borrow();
        **round.to_account_info().lamports.borrow_mut() = 0;
        Ok(())
    }

    pub fn genesis_lock(ctx: Context<GenesisLockContext>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());

        // TODO: check that executed after initialize + interval
        let config = &mut ctx.accounts.config;
        config.genesis_lock_once = true;

        let round = &mut ctx.accounts.round;
        let price = get_price(&mut ctx.accounts.price_update)?;
        round.lock_price = price.price;
        round.lock_price_exponent = price.exponent;
        config.current_epoch = round.epoch;

        let next_round = &mut ctx.accounts.next_round;

        emit!(RoundStarted {
            key: round.key(),
            epoch: round.epoch,
            close_ts: round.close_ts,
            lock_price: round.lock_price,
            lock_price_exponent: round.lock_price_exponent,
        });

        let future_round = &mut ctx.accounts.future_round;

        initialize_round(
            future_round,
            config.current_epoch + 2,
            config.interval_seconds,
            next_round.start_ts + config.interval_seconds as i64,
        )?;

        config.last_available_epoch = future_round.epoch;

        Ok(())
    }

    pub fn genesis_execute(ctx: Context<ExecuteRoundContext>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());

        let round = &mut ctx.accounts.round;
        let next_round = &mut ctx.accounts.next_round;
        let future_round = &mut ctx.accounts.future_round;
        let config = &mut ctx.accounts.config;
        config.current_epoch = next_round.epoch;

        let price = get_price(&mut ctx.accounts.price_update)?;

        // set current round
        round.close_price = price.price;
        round.close_price_exponent = price.exponent;

        calculate_rewards(round, config)?;

        emit!(RoundClosed {
            key: round.key(),
            epoch: round.epoch,
            close_ts: round.close_ts,
            close_price: round.close_price,
            close_price_exponent: round.close_price_exponent,
        });

        next_round.lock_price = price.price;
        next_round.lock_price_exponent = price.exponent;

        emit!(RoundStarted {
            key: next_round.key(),
            epoch: next_round.epoch,
            close_ts: next_round.close_ts,
            lock_price: next_round.lock_price,
            lock_price_exponent: next_round.lock_price_exponent,
        });

        initialize_round(
            future_round,
            config.current_epoch + 2,
            config.interval_seconds,
            next_round.lock_ts + config.interval_seconds as i64,
        )?;

        config.last_available_epoch = future_round.epoch;

        Ok(())
    }

    #[access_control(validate_claim(&ctx))]
    pub fn close_bet(ctx: Context<CloseBetContext>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());

        let bet = &mut ctx.accounts.bet;
        let round = &mut ctx.accounts.round;
        let user = &ctx.accounts.user;

        let reward: u64;

        // TODO: define when users should get a refund - conditions still need to be agreed
        if round.close_price > 0 {
            // Check if this bet is on the winning side
            let is_winner = if round.close_price > round.lock_price {
                bet.position == Position::Bull // Bulls win
            } else if round.close_price < round.lock_price {
                bet.position == Position::Bear // Bears win
            } else {
                false // Tie - house wins, no one gets rewards
            };

            if is_winner && round.reward_base > 0 {
                // Calculate reward for winners only
                reward = bet
                    .amount
                    .checked_mul(round.reward_amount)
                    .and_then(|v| v.checked_div(round.reward_base))
                    .ok_or(ErrorCode::MathOverflow)?;
            } else {
                // Loser or no winners - no reward
                reward = 0;
            }
        } else {
            reward = bet.amount; // Refund case
        }

        if reward > 0 {
            let treasury_seeds: &[&[u8]] = &[b"treasury", &[ctx.bumps.treasury]];
            let signer: &[&[&[u8]]] = &[treasury_seeds];

            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts,
                signer,
            );

            anchor_lang::system_program::transfer(cpi_ctx, reward)?;
        }

        let winning_position = if round.close_price > round.lock_price {
            Position::Bull
        } else if round.close_price < round.lock_price {
            Position::Bear
        } else {
            // House wins, no winner
            bet.position // or Position::Bull, but bet.position is fine for draw
        };

        let payout_ratio = if bet.amount > 0 {
            reward as f64 / bet.amount as f64
        } else {
            0.0
        };

        emit!(Claim {
            user: user.key(),
            epoch: round.epoch,
            amount: reward,
            winning_position,
            payout_ratio,
        });

        round.unresolved_bets_count = round
            .unresolved_bets_count
            .checked_sub(1)
            .ok_or(ErrorCode::Overflow)?;

        Ok(())
    }

    pub fn pause(ctx: Context<PauseContext>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());

        emit!(Pause {
            epoch: ctx.accounts.config.current_epoch,
        });
        Ok(())
    }

    pub fn unpause(ctx: Context<PauseContext>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key());

        let config = &mut ctx.accounts.config;
        config.genesis_lock_once = false;
        config.genesis_initialized = false;
        emit!(Unpause {
            epoch: config.current_epoch,
        });
        Ok(())
    }

    pub fn bet(
        ctx: Context<BetContext>,
        epoch: u64,
        position: Position,
        amount: u64,
    ) -> Result<()> {
        let round: &Account<'_, Round> = &ctx.accounts.round;
        let now = Clock::get()?.unix_timestamp;

        // Prevent betting within BETTING_CUTOFF seconds before lock_ts
        if now >= round.lock_ts - BETTING_CUTOFF as i64 {
            return Err(error!(ErrorCode::BettingClosed));
        }

        let rent = Rent::get()?;
        let space = 8 + Bet::INIT_SPACE;
        let lamports = rent.minimum_balance(space);

        let bet_info = ctx.accounts.bet.to_account_info();
        let treasury_info = ctx.accounts.treasury.to_account_info();

        // Check if the amount is sufficient
        if lamports > amount {
            return Err(error!(ErrorCode::BetTooSmall));
        }

        // Transfer amount to treasury
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        // Seeds for the bet PDA (must match the #[account] seeds)
        let bet_seeds: &[&[u8]] = &[
            b"bet",
            ctx.accounts.user.key.as_ref(),
            &epoch.to_le_bytes(),
            &[ctx.bumps.bet],
        ];
        let treasury_seeds: &[&[u8]] = &[b"treasury", &[ctx.bumps.treasury]];

        // Create the bet account with treasury as payer
        let create_account_ix = solana_program::system_instruction::create_account(
            &ctx.accounts.treasury.key(),
            &ctx.accounts.bet.key(),
            lamports,
            space as u64,
            ctx.program_id,
        );
        solana_program::program::invoke_signed(
            &create_account_ix,
            &[
                treasury_info.clone(),
                bet_info.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[treasury_seeds, bet_seeds],
        )?;

        // Now initialize the bet struct and write the Anchor discriminator (manual for older Anchor)
        let mut bet_data = ctx.accounts.bet.data.borrow_mut();
        let disc = anchor_lang::solana_program::hash::hashv(&[b"account:Bet"]);
        bet_data[..8].copy_from_slice(&disc.to_bytes()[..8]);
        let mut bet = Bet {
            user: ctx.accounts.user.key(),
            epoch,
            position,
            amount,
            claimed: false,
        };

        // Get mutable reference to round account
        let round = &mut ctx.accounts.round;
        update_round_and_bet(round, &mut bet, &mut ctx.accounts.user, position, amount)?;
        bet.serialize(&mut &mut bet_data[8..])?;

        Ok(())
    }

    pub fn withdraw_treasury(ctx: Context<WithdrawTreasuryContext>, amount: u64) -> Result<()> {
        use std::str::FromStr;
        let admin_pubkey = Pubkey::from_str(ADMIN_PUBKEY_STR).unwrap();
        require!(
            ctx.accounts.admin.key() == admin_pubkey,
            ErrorCode::UnauthorizedOperator
        );

        let config = &mut ctx.accounts.config;

        // Check if requested amount is available in treasury
        require!(
            amount <= config.treasury_amount,
            ErrorCode::InsufficientTreasuryFunds
        );

        // Transfer funds from treasury to admin
        let treasury_seeds: &[&[u8]] = &[b"treasury", &[ctx.bumps.treasury]];
        let signer: &[&[&[u8]]] = &[treasury_seeds];

        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: ctx.accounts.admin.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        // Deduct the withdrawn amount from treasury_amount
        config.treasury_amount = config
            .treasury_amount
            .checked_sub(amount)
            .ok_or(ErrorCode::Overflow)?;

        Ok(())
    }

    // TODO: add an instruction to transfer vault (treasury amount) to the admin wallet
}

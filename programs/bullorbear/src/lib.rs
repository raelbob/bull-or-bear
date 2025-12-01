#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::Discriminator;

declare_id!("DLEQCwxqxavJUK93bpdqXGkqJSmwJTmL2vnXRTPYNUau");

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

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_admin_or_operator!(ctx.accounts.payer.key(), config);
        config.locked_once = false;
        config.paused = false;
        config.current_epoch = match config.last_available_epoch {
            0 => 0,
            n => n + 1,
        };

        // Get the next full minute timestamp
        let next_full_minute = get_next_full_minute()?;

        let round = &mut ctx.accounts.round;
        round.epoch = config.current_epoch;
        round.start_ts = next_full_minute;
        round.lock_ts = next_full_minute + config.interval_seconds as i64;
        round.close_ts = next_full_minute + (config.interval_seconds as i64 * 2);

        emit!(Initialized {
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
        ctx: Context<ConfigInitializeContext>,
        interval: u16,
        min_bet_amount: u64,
        treasury_fee: u16,
        admin: Pubkey,
        operator: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(treasury_fee <= MAX_TREASURY_FEE, ErrorCode::InvalidFee);
        require!(
            admin != Pubkey::default() && operator != Pubkey::default(),
            ErrorCode::InvalidAdminOrOperator
        );

        config.locked_once = false;
        config.paused = false;
        config.interval_seconds = interval;
        config.min_bet_amount = min_bet_amount;
        config.treasury_fee = treasury_fee;
        config.admin = admin;
        config.operator = operator;
        Ok(())
    }

    pub fn config_update(
        ctx: Context<UpdateConfigContext>,
        admin: Option<Pubkey>,
        operator: Option<Pubkey>,
        interval_seconds: Option<u16>,
        min_bet_amount: Option<u64>,
        treasury_fee: Option<u16>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_admin!(ctx.accounts.payer.key(), config);

        if let Some(admin) = admin {
            config.admin = admin;
        }
        if let Some(operator) = operator {
            config.operator = operator;
        }
        if let Some(interval) = interval_seconds {
            config.interval_seconds = interval;
        }
        if let Some(min_bet) = min_bet_amount {
            config.min_bet_amount = min_bet;
        }
        if let Some(fee) = treasury_fee {
            require!(fee <= MAX_TREASURY_FEE, ErrorCode::InvalidFee);
            config.treasury_fee = fee;
        }
        Ok(())
    }

    pub fn close_config(ctx: Context<CloseConfigContext>) -> Result<()> {
        let config = &ctx.accounts.config;
        require_admin!(ctx.accounts.destination.key(), config);

        Ok(())
    }

    pub fn close_round(ctx: Context<CloseRoundContext>, _epoch: u64) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.destination.key(), &ctx.accounts.config);

        let round = &mut ctx.accounts.round;

        require_eq!(
            round.unresolved_bets_count,
            0,
            ErrorCode::UnresolvedBetsExist
        );

        Ok(())
    }

    pub fn round_lock(ctx: Context<LockRoundContext>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_not_paused(config)?;
        require_admin_or_operator!(ctx.accounts.payer.key(), config);

        let round = &mut ctx.accounts.round;
        // get price, make sure it matches lock timestamp of the current round
        let price = get_price(&mut ctx.accounts.price_update, round.lock_ts)?;
        round.lock_price = Some(price.price);
        round.lock_price_exponent = Some(price.exponent);

        emit!(RoundStarted {
            key: round.key(),
            epoch: round.epoch,
            close_ts: round.close_ts,
            lock_price: round.lock_price.unwrap(),
            lock_price_exponent: round.lock_price_exponent.unwrap(),
        });

        let last_available_round = &mut ctx.accounts.last_available_round;
        let future_round = &mut ctx.accounts.future_round;

        let mut future_start_ts = last_available_round.start_ts + config.interval_seconds as i64;
        let now = Clock::get()?.unix_timestamp;
        // Ensure future round starts in the future, so that we avoid rounds that can't be bet on
        if future_start_ts < now {
            future_start_ts = get_next_full_minute()?;
        }

        initialize_round(
            future_round,
            config.last_available_epoch + 1,
            config.interval_seconds,
            future_start_ts,
        )?;

        config.locked_once = true;
        config.current_epoch = round.epoch;
        config.last_available_epoch = future_round.epoch;

        Ok(())
    }

    pub fn round_execute(ctx: Context<ExecuteRoundContext>) -> Result<()> {
        require_admin_or_operator!(ctx.accounts.payer.key(), &ctx.accounts.config);

        let round = &mut ctx.accounts.round;
        let next_round = &mut ctx.accounts.next_round;
        let last_available_round = &mut ctx.accounts.last_available_round;
        let future_round = &mut ctx.accounts.future_round;
        let config = &mut ctx.accounts.config;

        // get price, make sure it matches close timestamp of the current round
        let price = get_price(&mut ctx.accounts.price_update, round.close_ts)?;

        // set current round
        round.close_price = Some(price.price);
        round.close_price_exponent = Some(price.exponent);

        calculate_rewards(round, config)?;

        next_round.lock_price = Some(price.price);
        next_round.lock_price_exponent = Some(price.exponent);

        let mut future_start_ts = last_available_round.start_ts + config.interval_seconds as i64;
        let now = Clock::get()?.unix_timestamp;
        // Ensure future round starts in the future, so that we avoid rounds that can't be bet on
        if future_start_ts < now {
            future_start_ts = get_next_full_minute()?;
        }

        initialize_round(
            future_round,
            config.last_available_epoch + 1,
            config.interval_seconds,
            future_start_ts,
        )?;

        config.current_epoch = next_round.epoch;
        config.last_available_epoch = future_round.epoch;

        emit!(RoundClosed {
            key: round.key(),
            epoch: round.epoch,
            close_ts: round.close_ts,
            close_price: round.close_price.unwrap(),
            close_price_exponent: round.close_price_exponent.unwrap(),
        });

        emit!(RoundStarted {
            key: next_round.key(),
            epoch: next_round.epoch,
            close_ts: next_round.close_ts,
            lock_price: next_round.lock_price.unwrap(),
            lock_price_exponent: next_round.lock_price_exponent.unwrap(),
        });

        Ok(())
    }

    pub fn round_add_future(ctx: Context<AddFutureRoundContext>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_admin_or_operator!(ctx.accounts.payer.key(), config);

        let last_available_round = &mut ctx.accounts.last_available_round;
        let future_round = &mut ctx.accounts.future_round;

        let mut future_start_ts = last_available_round.start_ts + config.interval_seconds as i64;
        let now = Clock::get()?.unix_timestamp;
        // Ensure future round starts in the future, so that we avoid rounds that can't be bet on
        if future_start_ts < now {
            future_start_ts = get_next_full_minute()?;
        }

        initialize_round(
            future_round,
            config.last_available_epoch + 1,
            config.interval_seconds,
            future_start_ts,
        )?;

        config.last_available_epoch = future_round.epoch;

        Ok(())
    }

    #[access_control(validate_claim(&ctx))]
    pub fn close_bet(ctx: Context<CloseBetContext>) -> Result<()> {
        // NOTE: This function can be called by ANYONE (user, admin, or operator)
        // This prevents DoS attacks where users intentionally don't claim.
        // Backend automatically processes unclaimed bets before closing rounds.
        // No permission checks needed - anyone can trigger claims
        let bet = &mut ctx.accounts.bet;
        let round = &mut ctx.accounts.round;
        let user = &ctx.accounts.user;
        let config = &mut ctx.accounts.config;

        let reward: u64;

        if round.close_price.is_some() && round.lock_price.is_some() {
            let close_price = round.close_price.unwrap();
            let lock_price = round.lock_price.unwrap();

            // Check if this bet is on the winning side
            let is_winner = if close_price > lock_price {
                bet.position == Position::Bull // Bulls win
            } else if close_price < lock_price {
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

        // First, update treasury tracking if there's a reward
        if reward > 0 {
            config.treasury_amount = config
                .treasury_amount
                .checked_sub(reward)
                .ok_or(ErrorCode::Overflow)?;
        }

        // Release pending bet amount (bet is now resolved)
        config.pending_bet_amount = config
            .pending_bet_amount
            .checked_sub(bet.amount)
            .ok_or(ErrorCode::Overflow)?;

        // Update round's unresolved bets count
        round.unresolved_bets_count = round
            .unresolved_bets_count
            .checked_sub(1)
            .ok_or(ErrorCode::Overflow)?;

        // NOW perform the transfer (after all accounting is updated)
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

        let winning_position =
            if let (Some(close_price), Some(lock_price)) = (round.close_price, round.lock_price) {
                if close_price > lock_price {
                    Position::Bull
                } else if close_price < lock_price {
                    Position::Bear
                } else {
                    // House wins, no winner
                    bet.position
                }
            } else {
                bet.position // Unresolved round
            };

        let payout_ratio_bps = if bet.amount > 0 {
            // Calculate ratio in basis points (10000 = 100%)
            reward
                .checked_mul(10000)
                .and_then(|v| v.checked_div(bet.amount))
                .unwrap_or(0)
        } else {
            0
        };

        emit!(Claim {
            user: user.key(),
            epoch: round.epoch,
            amount: reward,
            winning_position,
            payout_ratio_bps,
        });

        Ok(())
    }

    pub fn pause(ctx: Context<PauseContext>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_admin_or_operator!(ctx.accounts.payer.key(), config);
        config.paused = true;

        emit!(Pause {
            epoch: config.current_epoch,
        });
        Ok(())
    }

    pub fn unpause(ctx: Context<PauseContext>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_admin_or_operator!(ctx.accounts.payer.key(), config);
        config.paused = false;
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
        let config = &mut ctx.accounts.config;
        require_not_paused(config)?;
        let round: &Account<'_, Round> = &ctx.accounts.round;
        let now = Clock::get()?.unix_timestamp;

        // NOTE: Epoch validation is implicit and sufficient:
        // 1. Round account must exist at this epoch (Anchor validates the PDA)
        // 2. Round must be initialized with valid lock_ts (or this check fails)
        // 3. Only active rounds have lock_ts > now, preventing bets on old/invalid rounds
        // FEATURE: Short betting cutoff is intentional for game dynamics.
        // Users can see pool state and make informed last-second decisions.
        // This creates excitement and allows strategic play based on pool imbalance.
        // The oracle price at lock_ts determines outcomes, not the pool distribution,
        // so this doesn't compromise fairness - it's part of the game strategy.
        if now >= round.lock_ts - BETTING_CUTOFF as i64 {
            return Err(error!(ErrorCode::BettingClosed));
        }

        let rent = Rent::get()?;
        let space = 8 + Bet::INIT_SPACE;
        let lamports = rent.minimum_balance(space);

        // Check if the bet amount meets the minimum requirement
        if amount < config.min_bet_amount {
            return Err(error!(ErrorCode::BetTooSmall));
        }

        // Check if the amount is sufficient (must cover rent at least)
        if amount < lamports {
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
        let bet_info = ctx.accounts.bet.to_account_info();
        let treasury_info = ctx.accounts.treasury.to_account_info();

        // NOTE: Manual account creation is INTENTIONAL - treasury pays rent, not users
        // This is a UX feature, not a bug. Minimum bet ensures economic viability.
        // Account validation after creation ensures safety.
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

        // Validate account was created successfully
        require!(
            bet_info.owner == ctx.program_id,
            ErrorCode::InvalidAccountOwner
        );
        require!(bet_info.data_len() == space, ErrorCode::InvalidAccountSize);

        // Initialize the bet struct with validation
        let mut bet_data = ctx.accounts.bet.data.borrow_mut();

        // Write discriminator with bounds checking
        let disc = anchor_lang::solana_program::hash::hashv(&[b"account:Bet"]);
        if bet_data.len() < 8 {
            return Err(error!(ErrorCode::InsufficientAccountData));
        }
        bet_data[..8].copy_from_slice(&disc.to_bytes()[..8]);

        // Create bet struct
        let mut bet = Bet {
            user: ctx.accounts.user.key(),
            epoch,
            position,
            amount,
        };

        // Get mutable reference to round account
        let round = &mut ctx.accounts.round;
        let config = &mut ctx.accounts.config;
        update_round_and_bet(
            round,
            &mut bet,
            &mut ctx.accounts.user,
            position,
            amount,
            config,
        )?;

        // Serialize with error handling
        bet.serialize(&mut &mut bet_data[8..])
            .map_err(|_| ErrorCode::BetSerializationFailed)?;

        // Verify the data was written correctly by checking key fields
        drop(bet_data);
        let verification_data = ctx.accounts.bet.data.borrow();
        if verification_data.len() < 8 + 32 {
            // At minimum: discriminator + user pubkey
            return Err(error!(ErrorCode::BetVerificationFailed));
        }

        // Verify discriminator is correct
        let written_disc = &verification_data[..8];
        if written_disc != &disc.to_bytes()[..8] {
            return Err(error!(ErrorCode::InvalidDiscriminator));
        }

        Ok(())
    }

    pub fn bet_refund(ctx: Context<BetRefundContext>) -> Result<()> {
        let bet = &ctx.accounts.bet;
        let round = &mut ctx.accounts.round;
        let user = &ctx.accounts.user;
        let config = &mut ctx.accounts.config;
        let now = Clock::get()?.unix_timestamp;

        // Validation: Round must not have been resolved AND at least 30 minutes must have passed
        require!(
            (round.lock_price.is_none() || round.close_price.is_none())
                && now >= round.close_ts + SECONDS_BEFORE_REFUND_AVAILABLE,
            ErrorCode::RefundNotYetAvailable
        );

        // Refund the bet amount to the user
        let treasury_seeds: &[&[u8]] = &[b"treasury", &[ctx.bumps.treasury]];
        let signer: &[&[&[u8]]] = &[treasury_seeds];

        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_lang::system_program::transfer(cpi_ctx, bet.amount)?;

        // Deduct refund from treasury tracking
        config.treasury_amount = config
            .treasury_amount
            .checked_sub(bet.amount)
            .ok_or(ErrorCode::Overflow)?;

        // Release pending bet amount
        config.pending_bet_amount = config
            .pending_bet_amount
            .checked_sub(bet.amount)
            .ok_or(ErrorCode::Overflow)?;

        // Revert the round state
        round.total_amount = round
            .total_amount
            .checked_sub(bet.amount)
            .ok_or(ErrorCode::Overflow)?;

        match bet.position {
            Position::Bull => {
                round.bull_amount = round
                    .bull_amount
                    .checked_sub(bet.amount)
                    .ok_or(ErrorCode::Overflow)?;
                round.bull_total_bets = round
                    .bull_total_bets
                    .checked_sub(1)
                    .ok_or(ErrorCode::Overflow)?;
            }
            Position::Bear => {
                round.bear_amount = round
                    .bear_amount
                    .checked_sub(bet.amount)
                    .ok_or(ErrorCode::Overflow)?;
                round.bear_total_bets = round
                    .bear_total_bets
                    .checked_sub(1)
                    .ok_or(ErrorCode::Overflow)?;
            }
        }

        round.unresolved_bets_count = round
            .unresolved_bets_count
            .checked_sub(1)
            .ok_or(ErrorCode::Overflow)?;

        Ok(())
    }

    pub fn withdraw_treasury(ctx: Context<WithdrawTreasuryContext>, amount: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require_admin!(ctx.accounts.admin.key(), config);

        // SAFETY: This calculation ensures user funds are always protected.
        // - treasury_amount: Total funds in treasury
        // - pending_bet_amount: Sum of all potential payouts (bets that could be claimed)
        // - withdrawable: Only the profit/fees that aren't allocated to users
        //
        // The pending_bet_amount is updated atomically:
        // - Increased when bets are placed or rounds are executed (for winners)
        // - Decreased when bets are claimed or refunded
        //
        // This prevents any race condition or front-running issues as user funds
        // are always reserved and cannot be withdrawn by admin.
        let withdrawable = config
            .treasury_amount
            .checked_sub(config.pending_bet_amount)
            .ok_or(ErrorCode::InsufficientTreasuryFunds)?;

        // Check if requested amount is available
        require!(amount <= withdrawable, ErrorCode::InsufficientTreasuryFunds);

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
}

use crate::state::{Bet, Config, Round};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct ConfigInitializeContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    // CHECK: Program's upgrade authority
    #[account(
        constraint = program_data.upgrade_authority_address == Some(payer.key()) 
        @ crate::errors::ErrorCode::UnauthorizedOperator
    )]
    pub program_data: Account<'info, ProgramData>,

    #[account(address = crate::ID)]
    pub program: Program<'info, crate::program::Bullorbear>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfigContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct CloseConfigContext<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump,
        close = destination
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub destination: Signer<'info>,
}

#[derive(Accounts, Debug)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", &(if config.last_available_epoch == 0 { config.current_epoch } else { config.last_available_epoch + 1 }).to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        init,
        payer = payer,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", &(if config.last_available_epoch == 0 { config.current_epoch + 1 } else { config.last_available_epoch + 2 }).to_le_bytes()[..]],
        bump
    )]
    pub next_round: Account<'info, Round>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Debug)]
pub struct CloseBetContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bet", &user.key().to_bytes()[..], &round.epoch.to_le_bytes()[..]],
        bump,
        close = treasury
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK: We check this account matches bet.user, so it's safe.
    #[account(mut, address = bet.user)]
    pub user: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"round", &bet.epoch.to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Debug)]
pub struct BetRefundContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bet", &user.key().to_bytes()[..], &round.epoch.to_le_bytes()[..]],
        bump,
        close = treasury
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK: We check this account matches bet.user, so it's safe.
    #[account(mut, address = bet.user)]
    pub user: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"round", &bet.epoch.to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Debug)]
pub struct PauseContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockRoundContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"round", &config.current_epoch.to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        mut,
        seeds = [b"round", &(config.last_available_epoch).to_le_bytes()[..]],
        bump
    )]
    pub last_available_round: Account<'info, Round>,

    #[account(
        init,
        payer = payer,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", &(config.last_available_epoch + 1).to_le_bytes()[..]],
        bump
    )]
    pub future_round: Account<'info, Round>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteRoundContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"round", &config.current_epoch.to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        mut,
        seeds = [b"round", &(config.current_epoch + 1).to_le_bytes()[..]],
        bump
    )]
    pub next_round: Account<'info, Round>,

    #[account(
        mut,
        seeds = [b"round", &(config.last_available_epoch).to_le_bytes()[..]],
        bump
    )]
    pub last_available_round: Account<'info, Round>,

    #[account(
        init,
        payer = payer,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", &(config.last_available_epoch + 1).to_le_bytes()[..]],
        bump
    )]
    pub future_round: Account<'info, Round>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddFutureRoundContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"round", &config.last_available_epoch.to_le_bytes()[..]],
        bump
    )]
    pub last_available_round: Account<'info, Round>,

    #[account(
        init,
        payer = payer,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", &(config.last_available_epoch + 1).to_le_bytes()[..]],
        bump
    )]
    pub future_round: Account<'info, Round>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(epoch: u64)]
pub struct CloseRoundContext<'info> {
    #[account(mut)]
    pub destination: Signer<'info>,

    #[account(
        mut,
        close = destination,
        seeds = [b"round", &epoch.to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
#[instruction(epoch: u64)]
pub struct BetContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bet", user.key().as_ref(), &epoch.to_le_bytes()[..]],
        bump
    )]
    /// CHECK: PDA manually created
    pub bet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"round", &epoch.to_le_bytes()[..]],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    /// CHECK: manually funded
    pub treasury: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawTreasuryContext<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

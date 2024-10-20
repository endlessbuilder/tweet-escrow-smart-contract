use anchor_lang::prelude::*;

use crate::constants::{DEAL_SEED, ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED};
use crate::error::TweetEscrowError;
use crate::{Deal, EscrowConfig};

#[derive(Accounts)]
pub struct TakerServedCtx<'info> {
    #[account(
        mut,
        constraint = backend_wallet.key() == escrow_config.backend_wallet @ TweetEscrowError::InvalidAuthority
    )]
    pub backend_wallet: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_CONFIG_SEED.as_bytes()],
        bump = escrow_config.bump,
    )]
    pub escrow_config: Box<Account<'info, EscrowConfig>>,

    /// CHECK: empty PDA, authority for token accounts
    #[account(
        seeds = [ESCROW_AUTHORITY_SEED.as_bytes()],
        bump = escrow_config.escrow_authority_bump
    )]
    pub escrow_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [DEAL_SEED.as_bytes(), deal.maker.as_ref(), deal.taker.as_ref()],
        bump = deal.bump,
    )]
    pub deal: Box<Account<'info, Deal>>,

    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, TakerServedCtx>) -> Result<()> {
    msg!(">>> mark as seller served");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let escrow_config = ctx.accounts.escrow_config.as_mut();

    let deal = ctx.accounts.deal.as_mut();

    require!(
        (current_timestamp - deal.maker_deposit_at) < escrow_config.taker_service_time_window,
        TweetEscrowError::ServiceTimeWindowExpired
    );
    require!(deal.is_maker_deposit, TweetEscrowError::NotDepositedYet);
    require!(
        !deal.is_taker_served,
        TweetEscrowError::ServedBySellerAlready
    );

    deal.is_taker_served = true;
    deal.is_withdrawal = true;

    Ok(())
}

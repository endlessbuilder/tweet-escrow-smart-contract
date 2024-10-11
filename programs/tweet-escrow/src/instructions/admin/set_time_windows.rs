use anchor_lang::prelude::*;

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED};
use crate::error::TweetEscrowError;
use crate::EscrowConfig;

#[derive(Accounts)]
pub struct SetTimeWindowsCtx<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_CONFIG_SEED.as_bytes()],
        bump = escrow_config.bump,
        constraint = escrow_config.admin == admin.key() @ TweetEscrowError::InvalidAuthority

    )]
    pub escrow_config: Box<Account<'info, EscrowConfig>>,

    /// CHECK: empty PDA, authority for token accounts
    #[account(
        seeds = [ESCROW_AUTHORITY_SEED.as_bytes()],
        bump = escrow_config.escrow_authority_bump
    )]
    pub escrow_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetTimeWindowsParams {
    pub new_buyer_deposit_time_window: i64,
    pub new_seller_service_time_window: i64,
    pub new_seller_withdraw_time_window: i64,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, SetTimeWindowsCtx>,
    params: &SetTimeWindowsParams,
) -> Result<()> {
    msg!(">>> set time windows");

    let escrow_config = ctx.accounts.escrow_config.as_mut();
    escrow_config.buyer_deposit_time_window = params.new_buyer_deposit_time_window;
    escrow_config.seller_service_time_window = params.new_seller_service_time_window;
    escrow_config.seller_withdraw_time_window = params.new_seller_withdraw_time_window;

    Ok(())
}


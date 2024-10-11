use anchor_lang::prelude::*;

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED};
use crate::EscrowConfig;

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = EscrowConfig::LEN,
        seeds = [ESCROW_CONFIG_SEED.as_bytes()],
        bump
    )]
    pub escrow_config: Box<Account<'info, EscrowConfig>>,

    /// CHECK: empty PDA, authority for token accounts
    #[account(
        seeds = [ESCROW_AUTHORITY_SEED.as_bytes()],
        bump
    )]
    pub escrow_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeConifgParams {
    pub admin: Pubkey,
    pub backend_wallet: Pubkey,

    pub buyer_deposit_time_window: i64,
    pub seller_service_time_window: i64,
    pub seller_withdraw_time_window: i64,

    pub fee_percentagte: u8,
    pub fee_wallet: Pubkey,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeCtx>,
    params: &InitializeConifgParams,
) -> Result<()> {
    msg!(">>> initialize tweet config");

    let escrow_config = ctx.accounts.escrow_config.as_mut();
    escrow_config.admin = params.admin;
    escrow_config.backend_wallet = params.backend_wallet;

    escrow_config.buyer_deposit_time_window = params.buyer_deposit_time_window;
    escrow_config.seller_service_time_window = params.seller_service_time_window;
    escrow_config.seller_withdraw_time_window = params.seller_withdraw_time_window;

    escrow_config.fee_percentagte = params.fee_percentagte;
    escrow_config.fee_wallet = params.fee_wallet;

    escrow_config.bump = ctx.bumps.escrow_config;
    escrow_config.escrow_authority_bump = ctx.bumps.escrow_authority;

    Ok(())
}

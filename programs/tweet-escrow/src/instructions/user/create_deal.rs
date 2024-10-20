use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED, DEAL_ESCROW_SEED, DEAL_SEED};
use crate::error::TweetEscrowError;
use crate::{EscrowConfig, Deal};

#[derive(Accounts)]
pub struct CreateDealCtx<'info> {
    #[account(mut)]
    pub backend_wallet: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_CONFIG_SEED.as_bytes()],
        bump = escrow_config.bump,
        constraint = escrow_config.backend_wallet == backend_wallet.key() @ TweetEscrowError::InvalidAuthority

    )]
    pub escrow_config: Box<Account<'info, EscrowConfig>>,

    /// CHECK: empty PDA, authority for token accounts
    #[account(
        seeds = [ESCROW_AUTHORITY_SEED.as_bytes()],
        bump = escrow_config.escrow_authority_bump
    )]
    pub escrow_authority: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub maker: AccountInfo<'info>,
    /// CHECK;
    #[account(mut)]
    pub taker: AccountInfo<'info>,

    #[account(
        init,
        payer = backend_wallet,
        space = Deal::LEN,
        seeds = [DEAL_SEED.as_bytes(), maker.key().as_ref(), taker.key().as_ref()],
        bump
    )]
    pub deal: Box<Account<'info, Deal>>,

    /// CHECK: 
    #[account(
        seeds = [DEAL_ESCROW_SEED.as_bytes(), deal.key().as_ref()],
        bump
    )]
    pub deal_escrow: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateDealParams {
    pay_token_mint: Pubkey,
    pub deal_price: u64,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateDealCtx>,
    params: &CreateDealParams,
) -> Result<()> {
    msg!(">>> create order");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;
    
    let deal = ctx.accounts.deal.as_mut();
    deal.bump = ctx.bumps.deal;
    deal.deal_escrow_bump = ctx.bumps.deal_escrow;
    deal.maker = ctx.accounts.maker.key();
    deal.taker = ctx.accounts.taker.key();
    deal.pay_token_mint = params.pay_token_mint;
    deal.price = params.deal_price;
    deal.deposited_amount = 0;
    deal.is_maker_deposit = false;
    deal.is_taker_served = false;
    deal.is_withdrawal = false;
    deal.is_completed = false;
    
    Ok(())
}

use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::constants::{DEAL_ESCROW_SEED, DEAL_SEED, ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED};
use crate::error::TweetEscrowError;
use crate::{Deal, EscrowConfig};

#[derive(Accounts)]
pub struct WithdrawOrderCtx<'info> {
    #[account(
        mut,
        constraint = backend_wallet.key() == escrow_config.backend_wallet @ TweetEscrowError::InvalidWithdrawAuthority
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

    /// CHECK:
    #[account(
        seeds = [DEAL_ESCROW_SEED.as_bytes(), deal.key().as_ref()],
        bump = deal.deal_escrow_bump
    )]
    pub deal_escrow: AccountInfo<'info>,

    /// CHECK
    #[account(
        mut,
        owner = deal.taker @ TweetEscrowError::InvalidTokenAccount
    )]
    pub taker_pay_account: AccountInfo<'info>,

    /// CHECK
    #[account(
        mut,
        owner = deal_escrow.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub deal_escrow_pay_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, WithdrawOrderCtx>) -> Result<()> {
    msg!(">>> withdraw from order escrow to taker");

    let escrow_config = ctx.accounts.escrow_config.as_mut();

    let deal = ctx.accounts.deal.as_mut();
    let is_served_by_taker = deal.is_taker_served;
    let is_withdrawal = deal.is_withdrawal;

    require!(
        is_served_by_taker && is_withdrawal,
        TweetEscrowError::OrderNotWithdrawal
    );
    require!(!deal.is_completed, TweetEscrowError::WithdrawedAlready);

    let withdraw_amount =
        deal.price * (100 as u64 - escrow_config.fee_percentagte as u64) / 100 as u64;

    // withdraw to taker
    escrow_config.transfer_tokens(
        ctx.accounts
            .deal_escrow_pay_account
            .clone()
            .to_account_info(),
        ctx.accounts.taker_pay_account.clone().to_account_info(),
        ctx.accounts.escrow_authority.clone().to_account_info(),
        ctx.accounts.token_program.clone().to_account_info(),
        withdraw_amount,
    )?;

    deal.is_completed = true;

    Ok(())
}

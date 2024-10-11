use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED, ORDER_ESCROW_SEED, ORDER_SEED};
use crate::error::TweetEscrowError;
use crate::{EscrowConfig, Order};

#[derive(Accounts)]
pub struct WithdrawOrderCtx<'info> {
    #[account(
        mut,
        constraint = seller.key() == order.seller @ TweetEscrowError::InvalidWithdrawAuthority
    )]
    pub seller: Signer<'info>,

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
        seeds = [ORDER_SEED.as_bytes(), order.seller.as_ref(), order.buyer.as_ref()],
        bump = order.bump,
    )]
    pub order: Box<Account<'info, Order>>,

    /// CHECK:
    #[account(
        seeds = [ORDER_ESCROW_SEED.as_bytes(), order.key().as_ref()],
        bump = order.order_escrow_bump
    )]
    pub order_escrow: AccountInfo<'info>,

    /// CHECK:
    #[account(
        mut,
        constraint = fee_wallet.key() == escrow_config.fee_wallet @ TweetEscrowError::InvaildFeeWallet
    )]
    pub fee_wallet: AccountInfo<'info>,

    /// CHECK:
    #[account(
        mut,
        owner = fee_wallet.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub fee_pay_account: Account<'info, TokenAccount>,

    /// CHECK
    #[account(
        mut,
        owner = seller.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub seller_pay_account: Account<'info, TokenAccount>,

    /// CHECK
    #[account(
        mut,
        owner = order_escrow.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub order_escrow_pay_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, WithdrawOrderCtx>) -> Result<()> {
    msg!(">>> withdraw from order escrow to seller");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let escrow_config = ctx.accounts.escrow_config.as_mut();

    let order = ctx.accounts.order.as_mut();
    let is_served_by_seller = order.is_seller_served;
    let is_withdrawal = order.is_withdrawal;

    require!(
        is_served_by_seller && is_withdrawal,
        TweetEscrowError::OrderNotWithdrawal
    );
    require!(
        (current_timestamp - order.seller_served_at) > escrow_config.seller_withdraw_time_window,
        TweetEscrowError::OrderNotWithdrawal
    );
    require!(!order.is_completed, TweetEscrowError::WithdrawedAlready);

    let withdraw_amount = order.deposited_amount * escrow_config.fee_percentagte as u64 / 100;
    let fee_amount = order.deposited_amount - withdraw_amount;

    // withdraw to seller
    escrow_config.transfer_tokens(
        ctx.accounts
            .order_escrow_pay_account
            .clone()
            .to_account_info(),
        ctx.accounts.seller_pay_account.clone().to_account_info(),
        ctx.accounts.escrow_authority.clone().to_account_info(),
        ctx.accounts.token_program.clone().to_account_info(),
        withdraw_amount,
    )?;

    // transfer fee to fee wallet
    escrow_config.transfer_tokens(
        ctx.accounts
            .order_escrow_pay_account
            .clone()
            .to_account_info(),
        ctx.accounts.fee_pay_account.clone().to_account_info(),
        ctx.accounts.escrow_authority.clone().to_account_info(),
        ctx.accounts.token_program.clone().to_account_info(),
        fee_amount,
    )?;

    order.is_completed = true;

    Ok(())
}

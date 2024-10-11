use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED, ORDER_ESCROW_SEED, ORDER_SEED};
use crate::error::TweetEscrowError;
use crate::{EscrowConfig, Order};

#[derive(Accounts)]
pub struct DepositOrderCtx<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

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
        constraint = order.buyer == buyer.key() @ TweetEscrowError::InvalidAuthority
    )]
    pub order: Box<Account<'info, Order>>,

    /// CHECK: 
    #[account(
        seeds = [ORDER_ESCROW_SEED.as_bytes(), order.key().as_ref()],
        bump = order.order_escrow_bump
    )]
    pub order_escrow: AccountInfo<'info>,

    /// CHECK
    #[account(
        mut,
        owner = buyer.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub buyer_pay_account: Account<'info, TokenAccount>,

    /// CHECK
    #[account(
        mut,
        owner = order_escrow.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub order_escrow_pay_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateOrderParams {
    pub deposit_amount: u64,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositOrderCtx>,
    params: &CreateOrderParams,
) -> Result<()> {
    msg!(">>> deposit to order");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let escrow_config = ctx.accounts.escrow_config.as_mut();

    let order = ctx.accounts.order.as_mut();
    let is_deposit_completed = order.is_buyer_deposited;

    require!(
        !is_deposit_completed,
        TweetEscrowError::OrderDepositedAlready
    );

    let amount_to_be_deposited = order.deposited_amount;

    if params.deposit_amount >= amount_to_be_deposited {
        escrow_config.transfer_tokens_from_user(
            ctx.accounts.buyer_pay_account.clone().to_account_info(),
            ctx.accounts
                .order_escrow_pay_account
                .clone()
                .to_account_info(),
            ctx.accounts.buyer.clone().to_account_info(),
            ctx.accounts.token_program.clone().to_account_info(),
            amount_to_be_deposited
        )?;
        order.deposited_amount += amount_to_be_deposited;
        order.is_buyer_deposited = true;
        order.buyer_deposited_at = current_timestamp;
    } else {
        escrow_config.transfer_tokens_from_user(
            ctx.accounts.buyer_pay_account.clone().to_account_info(),
            ctx.accounts
                .order_escrow_pay_account
                .clone()
                .to_account_info(),
            ctx.accounts.buyer.clone().to_account_info(),
            ctx.accounts.token_program.clone().to_account_info(),
            params.deposit_amount,
        )?;
        order.deposited_amount += params.deposit_amount;
    }

    Ok(())
}

use anchor_lang::prelude::*;

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED, ORDER_SEED};
use crate::error::TweetEscrowError;
use crate::{EscrowConfig, Order};

#[derive(Accounts)]
pub struct SellerServedCtx<'info> {
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

    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, SellerServedCtx>) -> Result<()> {
    msg!(">>> mark as seller served");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let escrow_config = ctx.accounts.escrow_config.as_mut();

    let order = ctx.accounts.order.as_mut();
    
    require!(
        (current_timestamp - order.buyer_deposited_at) < escrow_config.seller_service_time_window,
        TweetEscrowError::ServiceTimeWindowExpired
    );
    require!(order.is_buyer_deposited, TweetEscrowError::NotDepositedYet);
    require!(!order.is_seller_served, TweetEscrowError::ServedBySellerAlready);

    order.is_seller_served = true;
    order.seller_served_at = current_timestamp;

    Ok(())
}

use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::constants::{ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED, ORDER_ESCROW_SEED, ORDER_SEED};
use crate::error::TweetEscrowError;
use crate::{EscrowConfig, Order};

#[derive(Accounts)]
pub struct CreateOrderCtx<'info> {
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
    pub seller: AccountInfo<'info>,
    /// CHECK;
    #[account(mut)]
    pub buyer: AccountInfo<'info>,

    #[account(
        init,
        payer = backend_wallet,
        space = Order::LEN,
        seeds = [ORDER_SEED.as_bytes(), seller.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub order: Box<Account<'info, Order>>,

    /// CHECK: 
    #[account(
        seeds = [ORDER_ESCROW_SEED.as_bytes(), order.key().as_ref()],
        bump
    )]
    pub order_escrow: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateOrderParams {
    pub order_price: u64,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateOrderCtx>,
    params: &CreateOrderParams,
) -> Result<()> {
    msg!(">>> create order");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;
    
    let order = ctx.accounts.order.as_mut();
    order.bump = ctx.bumps.order;
    order.order_escrow_bump = ctx.bumps.order_escrow;
    order.seller = ctx.accounts.seller.key();
    order.buyer = ctx.accounts.buyer.key();
    order.price = params.order_price;
    order.deposited_amount = 0;
    order.seller_approved_at = current_timestamp;
    order.is_buyer_deposited = false;
    order.is_seller_served = false;
    order.is_withdrawal = false;
    
    Ok(())
}

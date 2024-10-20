use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::constants::{DEAL_ESCROW_SEED, DEAL_SEED, ESCROW_AUTHORITY_SEED, ESCROW_CONFIG_SEED};
use crate::error::TweetEscrowError;
use crate::{Deal, EscrowConfig};

#[derive(Accounts)]
pub struct DepositOrderCtx<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

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
        constraint = deal.maker == maker.key() @ TweetEscrowError::InvalidAuthority
    )]
    pub deal: Box<Account<'info, Deal>>,

    /// CHECK
    #[account(
        mut,
        owner = deal_escrow.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub deal_escrow_pay_account: AccountInfo<'info>,

    /// CHECK:
    #[account(
        seeds = [DEAL_ESCROW_SEED.as_bytes(), deal.key().as_ref()],
        bump = deal.deal_escrow_bump
    )]
    pub deal_escrow: AccountInfo<'info>,

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
    pub fee_pay_account: AccountInfo<'info>,

    /// CHECK
    #[account(
        mut,
        owner = maker.key() @ TweetEscrowError::InvalidTokenAccount
    )]
    pub maker_pay_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositOrderParams {
    pub deposit_amount: u64,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositOrderCtx>,
    params: &DepositOrderParams,
) -> Result<()> {
    msg!(">>> deposit to order");

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let escrow_config = ctx.accounts.escrow_config.as_mut();

    let deal = ctx.accounts.deal.as_mut();

    require!(
        !deal.is_maker_deposit,
        TweetEscrowError::DealDepositedAlready
    );

    let amount_to_be_deposited = deal.price - deal.deposited_amount;

    if params.deposit_amount >= amount_to_be_deposited {
        escrow_config.transfer_tokens_from_user(
            ctx.accounts.maker_pay_account.clone().to_account_info(),
            ctx.accounts
                .deal_escrow_pay_account
                .clone()
                .to_account_info(),
            ctx.accounts.maker.clone().to_account_info(),
            ctx.accounts.token_program.clone().to_account_info(),
            amount_to_be_deposited,
        )?;
        deal.deposited_amount += amount_to_be_deposited;
        deal.is_maker_deposit = true;
        deal.maker_deposit_at = current_timestamp;

        let fee_amount = deal.price * (escrow_config.fee_percentagte as u64) / 100;

        // transfer fee to fee wallet
        escrow_config.transfer_tokens(
            ctx.accounts
                .deal_escrow_pay_account
                .clone()
                .to_account_info(),
            ctx.accounts.fee_pay_account.clone().to_account_info(),
            ctx.accounts.escrow_authority.clone().to_account_info(),
            ctx.accounts.token_program.clone().to_account_info(),
            fee_amount,
        )?;
    } else {
        escrow_config.transfer_tokens_from_user(
            ctx.accounts.maker_pay_account.clone().to_account_info(),
            ctx.accounts
                .deal_escrow_pay_account
                .clone()
                .to_account_info(),
            ctx.accounts.maker.clone().to_account_info(),
            ctx.accounts.token_program.clone().to_account_info(),
            params.deposit_amount,
        )?;
        deal.deposited_amount += params.deposit_amount;
    }

    Ok(())
}

mod constants;
mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;

use instructions::*;
use state::*;

declare_id!("Bh3Y47AqrAWBGCtFmSKqBdyA7tNtHGdD2yrYF9apSWJH");

#[program]
pub mod tweet_escrow {
    use super::*;

    // admin

    pub fn initialize<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCtx>,
        params: InitializeConifgParams,
    ) -> Result<()> {
        instructions::admin::initialize::handler(ctx, &params)
    }

    pub fn set_admin<'info>(
        ctx: Context<'_, '_, '_, 'info, SetAdminCtx>,
        params: SetAdminParams,
    ) -> Result<()> {
        instructions::admin::set_admin::handler(ctx, &params)
    }

    pub fn set_backend_wallet<'info>(
        ctx: Context<'_, '_, '_, 'info, SetBackendWalletCtx>,
        params: SetBackendWalletParams,
    ) -> Result<()> {
        instructions::admin::set_backend_wallet::handler(ctx, &params)
    }

    pub fn set_fee_wallet<'info>(
        ctx: Context<'_, '_, '_, 'info, SetFeeWalletCtx>,
        params: SetFeeWalletParams,
    ) -> Result<()> {
        instructions::admin::set_fee_wallet::handler(ctx, &params)
    }

    pub fn set_fee_percent<'info>(
        ctx: Context<'_, '_, '_, 'info, SetFeePercentCtx>,
        params: SetFeePercentParams,
    ) -> Result<()> {
        instructions::admin::set_fee_percent::handler(ctx, &params)
    }

    pub fn set_time_windows<'info>(
        ctx: Context<'_, '_, '_, 'info, SetTimeWindowsCtx>,
        params: SetTimeWindowsParams,
    ) -> Result<()> {
        instructions::admin::set_time_windows::handler(ctx, &params)
    }

    // backend

    pub fn create_deal<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateDealCtx>,
        params: CreateDealParams,
    ) -> Result<()> {
        instructions::backend::create_deal::handler(ctx, &params)
    }

    pub fn seller_served<'info>(ctx: Context<'_, '_, '_, 'info, TakerServedCtx>) -> Result<()> {
        instructions::taker_served::handler(ctx)
    }

    pub fn withdraw_deal<'info>(ctx: Context<'_, '_, '_, 'info, WithdrawDealCtx>) -> Result<()> {
        instructions::backend::withdraw_deal::handler(ctx)
    }

    // user

    pub fn deposit_deal<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositDealCtx>,
        params: DepositDealParams,
    ) -> Result<()> {
        instructions::user::deposit_deal::handler(ctx, &params)
    }
}

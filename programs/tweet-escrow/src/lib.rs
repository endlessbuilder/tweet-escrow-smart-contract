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

    pub fn initialize<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCtx>,
        params: InitializeConifgParams,
    ) -> Result<()> {
        initialize::handler(ctx, &params)
    }

    pub fn set_admin<'info>(
        ctx: Context<'_, '_, '_, 'info, SetAdminCtx>,
        params: SetAdminParams,
    ) -> Result<()> {
        set_admin::handler(ctx, &params)
    }

    pub fn set_backend_wallet<'info>(
        ctx: Context<'_, '_, '_, 'info, SetBackendWalletCtx>,
        params: SetBackendWalletParams,
    ) -> Result<()> {
        set_backend_wallet::handler(ctx, &params)
    }

    pub fn set_fee_wallet<'info>(
        ctx: Context<'_, '_, '_, 'info, SetFeeWalletCtx>,
        params: SetFeeWalletParams,
    ) -> Result<()> {
        set_fee_wallet::handler(ctx, &params)
    }

    pub fn set_fee_percent<'info>(
        ctx: Context<'_, '_, '_, 'info, SetFeePercentCtx>,
        params: SetFeePercentParams,
    ) -> Result<()> {
        set_fee_percent::handler(ctx, &params)
    }

    pub fn set_time_windows<'info>(
        ctx: Context<'_, '_, '_, 'info, SetTimeWindowsCtx>,
        params: SetTimeWindowsParams,
    ) -> Result<()> {
        set_time_windows::handler(ctx, &params)
    }

    pub fn create_order<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateDealCtx>,
        params: CreateDealParams,
    ) -> Result<()> {
        create_deal::handler(ctx, &params)
    }

    pub fn seller_served<'info>(
        ctx: Context<'_, '_, '_, 'info, TakerServedCtx>,
    ) -> Result<()> {
        taker_served::handler(ctx)
    }

    pub fn deposit_order<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositDealCtx>,
        params: DepositDealParams,
    ) -> Result<()> {
        deposit_deal::handler(ctx, &params)
    }

    pub fn withdraw_order<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawOrderCtx>,
    ) -> Result<()> {
        withdraw_order::handler(ctx)
    }

}

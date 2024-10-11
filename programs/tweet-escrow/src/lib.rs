pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

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
}

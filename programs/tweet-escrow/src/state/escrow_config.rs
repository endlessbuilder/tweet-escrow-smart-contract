use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

use crate::constants::ESCROW_AUTHORITY_SEED;

#[account]
#[derive(Default, Debug)]
pub struct EscrowConfig {
    pub admin: Pubkey,
    pub backend_wallet: Pubkey,

    pub taker_service_time_window: i64,

    pub fee_percentagte: u8,
    pub fee_wallet: Pubkey,

    pub bump: u8,
    pub escrow_authority_bump: u8,

    _reserved: [u128; 8],
}

impl EscrowConfig {
    pub const LEN: usize = 8 + std::mem::size_of::<EscrowConfig>();

    pub fn transfer_tokens<'info>(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        let authority_seeds: &[&[&[u8]]] =
            &[&[ESCROW_AUTHORITY_SEED.as_bytes(), &[self.escrow_authority_bump]]];

        let context = CpiContext::new(
            token_program,
            Transfer {
                from,
                to,
                authority,
            },
        )
        .with_signer(authority_seeds);

        anchor_spl::token::transfer(context, amount)
    }

    pub fn transfer_tokens_from_user<'info>(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        let context = CpiContext::new(
            token_program,
            Transfer {
                from,
                to,
                authority,
            },
        );
        anchor_spl::token::transfer(context, amount)
    }
}

use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Deal {
    pub maker: Pubkey, // maker lists tweet request
    pub taker: Pubkey, // toker tweets

    pub price: u64, // deal price
    pub pay_token_mint: Pubkey, // payment token mint for order
    
    pub maker_deposit_at: i64, // timestamp when deal started, deal start once maker deposit amount to contract

    pub deposited_amount: u64, // amount
    pub is_maker_deposit: bool, 
    pub is_taker_served: bool,
    pub is_withdrawal: bool,

    pub is_completed: bool,

    pub bump: u8,
    pub deal_escrow_bump: u8,
}

impl Deal {
    pub const LEN: usize = 8 + std::mem::size_of::<Deal>();
}
use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Order {
    pub seller: Pubkey,
    pub buyer: Pubkey,

    pub price: u64, // order price
    
    pub seller_approved_at: i64, // timestamp when seller accepts buyer's order
    pub buyer_deposited_at: i64, // timestamp when buyer deposit full amount

    pub is_buyer_deposited: bool, 
    pub is_seller_served: bool,
    pub is_withdrawal: bool,

    pub bump: u8
}

impl Order {
    pub const LEN: usize = 8 + std::mem::size_of::<Order>();
}
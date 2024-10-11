use anchor_lang::prelude::*;

#[error_code]
pub enum TweetEscrowError {
    #[msg("Signer authority is invalid")]
    InvalidAuthority,
    #[msg("Token account is invalid")]
    InvalidTokenAccount,
    #[msg("Order is already depositted by buyer")]
    OrderDepositedAlready,
}

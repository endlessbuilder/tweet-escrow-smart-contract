use anchor_lang::prelude::*;

#[error_code]
pub enum TweetEscrowError {
    #[msg("Signer authority is invalid")]
    InvalidAuthority,
}

use anchor_lang::prelude::*;

#[error_code]
pub enum TweetEscrowError {
    #[msg("Signer authority is invalid")]
    InvalidAuthority,
    #[msg("Token account is invalid")]
    InvalidTokenAccount,
    #[msg("Order is already depositted by buyer")]
    OrderDepositedAlready,
    #[msg("Withdrawer is not invalid for this order")]
    InvalidWithdrawAuthority,
    #[msg("Fee wallet is invalid")]
    InvaildFeeWallet,
    #[msg("Deposit time window has expired")]
    DepositTimeWindowExpired,
    #[msg("Order is not withdrawal")]
    OrderNotWithdrawal,
    #[msg("Withdraw time window has not passed")]
    WithdrawTimewindowNotPassedYet,
    #[msg("Withdraw is already done by seller")]
    WithdrawedAlready,
    #[msg("Service time window has expired")]
    ServiceTimeWindowExpired,
    #[msg("Order has not been deposited by buyer yet")]
    NotDepositedYet,
    #[msg("Order has been already served by seller yet")]
    ServedBySellerAlready,
}

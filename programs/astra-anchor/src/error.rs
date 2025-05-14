use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("invalid mint")]
    InvalidMint,
    #[msg("insufficient funds")]
    InsufficientFunds
}

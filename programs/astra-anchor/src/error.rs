use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Math overflow error")]
    MathOverflow,
    #[msg("insufficient funds")]
    InsufficientFunds,
}

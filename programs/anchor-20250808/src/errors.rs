use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid account provided")]
    InvalidAccount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
}
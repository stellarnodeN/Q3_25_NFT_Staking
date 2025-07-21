use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Time has not yet elapsed")]
    TimeNotElapsed,
    #[msg("Maximum stake limit reached")]
    MaxStake,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Arithmetic overflow")]
    Overflow,
}

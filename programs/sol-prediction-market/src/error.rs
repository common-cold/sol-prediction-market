use anchor_lang::prelude::*;

#[error_code]
pub enum SolPredictionError {
    #[msg("Invalid Outcome value provided")]
    InvalidOutcome,

    #[msg("Market Not Settled")]
    MarketNotSettled,

    #[msg("Winning Outcome is not set")]
    WinningOutcomeNotSet,

    #[msg("Market has already settled")]
    MarketAlreadySettled,
}
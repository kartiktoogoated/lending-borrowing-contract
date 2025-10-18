use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {
    #[msg("Invalid Account Owner")]
    InvalidAmount,
    #[msg("Unauthorized Operation")]
    Unauthorized,
    #[msg("Market Account Mismatch")]
    MarketMismatch,
    #[msg("Insufficient Liquidity in Reserve")]
    NoLiquidity,
    #[msg("Health Factor too low for this action")]
    UnhealhtyObligation,
    #[msg("Oracle price is invalid or stale")]
    InvalidOraclePrice,
}

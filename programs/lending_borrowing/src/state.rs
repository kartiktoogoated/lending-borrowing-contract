use anchor_lang::prelude::*;

#[account]
pub struct LendingMarket {
    pub bump: u8,
    pub owner: Pubkey,
    pub quote_currency: [u8; 32],
    pub token_program: Pubkey,
    pub pyth_program: Pubkey,
}

#[account]
pub struct Reserve {
    pub bump: u8,
    pub lending_market: Pubkey,
    pub liquidity_mint: Pubkey,
    pub liquidity_supply_vault: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_supply_vault: Pubkey,
    pub pyth_price_account: Pubkey,
    pub liquidity_available_amount: u64,
    pub liquidity_borrowed_amount: u128,
    pub config: ReserveConfig,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReserveConfig {
    pub loan_to_value_ratio_bps: u64,
    pub liquidation_threshold_bps: u64,
    pub optimal_utilization_rate_bps: u64,
    pub min_borrow_rate_bps: u64,
    pub max_borrow_rate_bps: u64,
}

#[account]
pub struct Obligation {
    pub bump: u8,
    pub lending_market: Pubkey,
    pub total_collateral_value: u128,
    pub total_borrow_value: u128,
    pub deposited_collateral: Vec<ObligationCollateral>,
    pub borrowed_liquidity: Vec<ObligationLiquidity>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ObligationCollateral {
    pub reserve: Pubkey,
    pub deposited_amount: u64,
    pub market_value_quote: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ObligationLiquidity {
    pub reserve: Pubkey,
    pub borrowed_amount_wads: u128,
    pub cumulative_borrow_rate_wads: u128,
    pub market_value_quote: u128,
}

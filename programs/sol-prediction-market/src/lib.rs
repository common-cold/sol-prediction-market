use anchor_lang::prelude::*;

declare_id!("DJ51Z4HfzNTxvTpKPpyM3JQK85yLc5FicCxxNJZr4Pkp");

pub mod instructions;
pub mod state;
use crate::instructions::*;

#[program]
pub mod sol_prediction_market {
    use super::*;

    pub fn initialize_market<'info> (ctx: Context<'_, '_, '_, 'info, InitializeMarket<'info>>, market_id: [u8; 12]) -> Result<()> {
        ctx.accounts.process(market_id, ctx.bumps.market_account)
    }

    pub fn split<'info> (ctx: Context<'_, '_, '_, 'info, Split<'info>>, market_id: [u8; 12], amount: u64) -> Result<()> {
        ctx.accounts.process(market_id, amount, ctx.bumps.market_account)
    }
}

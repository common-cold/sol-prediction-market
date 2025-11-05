use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum WinningOutcome {
    OutcomeA,
    OutcomeB,
    Neither
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    pub market_id: [u8; 12],
    pub outcome_a_mint: Pubkey,
    pub outcome_b_mint: Pubkey,
    pub base_token_mint: Pubkey,
    pub base_token_vault: Pubkey,
    pub is_settled: bool,
    pub winning_outcome: Option<WinningOutcome>,
    pub bump: u8
}
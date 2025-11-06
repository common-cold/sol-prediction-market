use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{Mint, TokenAccount}};

use crate::state::Market;

#[derive(Accounts)]
#[instruction(market_id: [u8; 12])]
pub struct InitializeMarket<'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space = 8 + Market::INIT_SPACE,
        payer = authority,
        seeds = [b"market".as_ref(), market_id.as_ref()],
        bump
    )]
    pub market_account: Account<'info, Market>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = market_account,
        seeds = [b"outcome_a".as_ref(), market_account.key().as_ref()],
        bump
    )]
    pub outcome_a_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = market_account,
        seeds = [b"outcome_b".as_ref(), market_account.key().as_ref()],
        bump
    )]
    pub outcome_b_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = outcome_a_mint,
        associated_token::authority = market_account,
        associated_token::token_program = token_program
    )]
    pub market_outcome_a_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = outcome_b_mint,
        associated_token::authority = market_account,
        associated_token::token_program = token_program
    )]
    pub market_outcome_b_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub base_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = base_token_mint,
        associated_token::authority = market_account,
        associated_token::token_program = token_program
    )]
    pub base_token_vault: Box<InterfaceAccount<'info, TokenAccount>>, 

    pub system_program: Program<'info, System>,
    
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> InitializeMarket<'info> {
    pub fn process(&mut self, market_id: [u8; 12], bump: u8) -> Result<()> {

        let market = &mut self.market_account;
        market.authority = self.authority.key();
        market.market_id = market_id;
        market.outcome_a_mint = self.outcome_a_mint.key();
        market.outcome_b_mint = self.outcome_b_mint.key();
        market.base_token_mint = self.base_token_mint.key();
        market.base_token_vault = self.base_token_vault.key();
        market.is_settled = false;
        market.winning_outcome = None;
        market.bump = bump;

        Ok(())
    }
}
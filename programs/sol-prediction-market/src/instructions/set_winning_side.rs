use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Burn, MintTo, SetAuthority, Token, TransferChecked, burn, mint_to, set_authority, spl_token::instruction::AuthorityType, transfer_checked}, token_interface::{Mint, TokenAccount}};

use crate::{error::SolPredictionError, state::{Market, WinningOutcome}};

#[derive(Accounts)]
#[instruction(market_id: [u8; 12])]
pub struct SetWinningSide<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market".as_ref(), market_id.as_ref()],
        bump,
        constraint = !market_account.is_settled @ SolPredictionError::MarketAlreadySettled
    )]
    pub market_account: Account<'info, Market>,

     #[account(
        mut,
        seeds = [b"outcome_a".as_ref(), market_account.key().as_ref()],
        bump
    )]
    pub outcome_a_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"outcome_b".as_ref(), market_account.key().as_ref()],
        bump
    )]
    pub outcome_b_mint: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,
    
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> SetWinningSide<'info> {
    pub fn process(&mut self, market_id: [u8; 12], market_bump: u8, winner: WinningOutcome) -> Result<()> {

        require!(
            matches!(winner, WinningOutcome::OutcomeA | WinningOutcome::OutcomeB),
            SolPredictionError::InvalidOutcome
        );

        let market_data = &mut self.market_account;
        
        //set winning outcome and is_settled = true
        market_data.winning_outcome = Some(winner);
        market_data.is_settled = true;

        let seeds: &[&[&[u8]]] = &[&[b"market".as_ref(), market_id.as_ref(), &[market_bump]]];

        //set mint authority of outcome A to None
        let remove_outcome_a_mint_authority_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            SetAuthority {
                current_authority: self.market_account.to_account_info(),
                account_or_mint: self.outcome_a_mint.to_account_info()
            }, 
            seeds
        );

        set_authority(
            remove_outcome_a_mint_authority_ctx, 
            AuthorityType::MintTokens, 
            None
        )?;


        //set mint authority of outcome B to None
        let remove_outcome_b_mint_authority_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            SetAuthority {
                current_authority: self.market_account.to_account_info(),
                account_or_mint: self.outcome_b_mint.to_account_info()
            }, 
            seeds
        );

        set_authority(
            remove_outcome_b_mint_authority_ctx, 
            AuthorityType::MintTokens, 
            None
        )?;

        Ok(())
    }
}
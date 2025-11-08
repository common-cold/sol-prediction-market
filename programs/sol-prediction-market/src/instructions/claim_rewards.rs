use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{associated_token::AssociatedToken, token::{Burn, Token, TransferChecked, burn, transfer, transfer_checked}, token_interface::{Mint, TokenAccount}};

use crate::{error::SolPredictionError, state::{Market, WinningOutcome}};

#[derive(Accounts)]
#[instruction(market_id: [u8; 12])]
pub struct ClaimRewards<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"market".as_ref(), market_id.as_ref()],
        bump,
        constraint = market_account.is_settled @ SolPredictionError::MarketNotSettled
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

    #[account(mut)]
    pub base_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub base_token_vault: Box<InterfaceAccount<'info, TokenAccount>>, 

    #[account(mut)]
    pub user_outcome_a_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_outcome_b_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = base_token_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_base_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> ClaimRewards<'info> {
    pub fn process(&mut self, market_id: [u8; 12], market_bump: u8) -> Result<()> {
        
        let market = &mut self.market_account;

        let winning_outcome = market.winning_outcome
            .ok_or_else(|| error!(SolPredictionError::WinningOutcomeNotSet))?;

        let outcome_a_amount = self.user_outcome_a_ata.amount;
        let outcome_b_amount = self.user_outcome_b_ata.amount;


        //burn outcome_a and outcome_b tokens from user
        burn(
            CpiContext::new(
                self.token_program.to_account_info(), 
                Burn {
                    mint: self.outcome_a_mint.to_account_info(),
                    from: self.user_outcome_a_ata.to_account_info(),
                    authority: self.user.to_account_info()
                }
            ), 
            outcome_a_amount
        )?;

        burn(
            CpiContext::new(
                self.token_program.to_account_info(), 
                Burn {
                    mint: self.outcome_b_mint.to_account_info(),
                    from: self.user_outcome_b_ata.to_account_info(),
                    authority: self.user.to_account_info()
                }
            ), 
            outcome_b_amount
        )?;


        let winning_amount = match winning_outcome {
            WinningOutcome::OutcomeA => outcome_a_amount,
            WinningOutcome::OutcomeB => outcome_b_amount,
            WinningOutcome::Neither => 0,
        };

        let seeds: &[&[&[u8]]] = &[&[b"market".as_ref(), market_id.as_ref(), &[market_bump]]];

        if winning_amount > 0 {
            let tranfer_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(), 
                TransferChecked {
                    from: self.base_token_vault.to_account_info(),
                    mint: self.base_token_mint.to_account_info(),
                    to: self.user_base_token_ata.to_account_info(),
                    authority: self.market_account.to_account_info()
                }, 
                seeds
            );

            transfer_checked(
                tranfer_ctx, 
                winning_amount, 
                self.base_token_mint.decimals
            )?;
        }

        Ok(())
    }
}
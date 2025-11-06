use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{MintTo, Token, TransferChecked, mint_to, transfer_checked}, token_interface::{Mint, TokenAccount}};

use crate::state::Market;

#[derive(Accounts)]
#[instruction(market_id: [u8; 12])]
pub struct Split<'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"market".as_ref(), market_id.as_ref()],
        bump
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

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = outcome_a_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_outcome_a_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = outcome_b_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
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

impl<'info> Split<'info> {
    pub fn process(&mut self, market_id: [u8; 12], amount: u64, market_bump: u8) -> Result<()> {
        
        //transfer base token from user to base token vault
        let base_transfer_ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            TransferChecked {
                from: self.user_base_token_ata.to_account_info(),
                mint: self.base_token_mint.to_account_info(),
                to: self.base_token_vault.to_account_info(),
                authority: self.user.to_account_info()
            }
        );
        transfer_checked(
            base_transfer_ctx, 
            amount, 
            self.base_token_mint.decimals
        )?;


        let seeds: &[&[&[u8]]] = &[&[b"market".as_ref(), market_id.as_ref(), &[market_bump]]];

        //mint equivalent outcome_a token to user

        let outcome_a_mint_to_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            MintTo {
                mint: self.outcome_a_mint.to_account_info(),
                to: self.user_outcome_a_ata.to_account_info(),
                authority: self.market_account.to_account_info()
            }, 
            seeds
        );

        mint_to(outcome_a_mint_to_ctx, amount)?;


        //mint equivalent outcome_b token to user

        let outcome_b_mint_to_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            MintTo {
                mint: self.outcome_b_mint.to_account_info(),
                to: self.user_outcome_b_ata.to_account_info(),
                authority: self.market_account.to_account_info()
            }, 
            seeds
        );

        mint_to(outcome_b_mint_to_ctx, amount)?;

        
        Ok(())
    }
}


use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked, close_account, CloseAccount}};

use crate::{ItemInfo, ITEM_SEED};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mint::decimals = 0,
        mint::token_program = token_program,
        mint::authority = signer
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer, 
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = signer
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [ITEM_SEED, item.owner.as_ref(), item.mint.as_ref()],
        bump = item.bump,
        close = signer,
        constraint = item.owner == signer.key(),
        constraint = item.mint == mint.key()
    )]
    pub item: Account<'info, ItemInfo>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = item,
    )]
    pub item_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Cancel<'info> {
    pub fn handler(&mut self) -> Result<()> {
        let seeds = &[
            ITEM_SEED,
            self.item.owner.as_ref(),
            self.item.mint.as_ref(),
            &[self.item.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(), 
                TransferChecked { 
                    from: self.item_ata.to_account_info(), 
                    mint: self.mint.to_account_info(), 
                    to: self.user_ata.to_account_info(), 
                    authority: self.item.to_account_info() 
                }, signer_seeds), 
                1, 
                self.mint.decimals
            )?;

        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(), 
                CloseAccount { 
                    account: self.item_ata.to_account_info(), 
                    destination: self.signer.to_account_info(), 
                    authority: self.item.to_account_info() 
                }, 
                signer_seeds)
            )?;
        Ok(())
    }
}

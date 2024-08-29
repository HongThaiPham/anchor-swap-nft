use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked}};

use crate::{ItemInfo, ITEM_SEED};

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut, 
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = signer
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        space = 8 + ItemInfo::INIT_SPACE,
        seeds = [ITEM_SEED, signer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub item: Account<'info, ItemInfo>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = item
    )]
    pub item_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    pub fn handler(&mut self, lamports: u64, bumps: &CreateBumps) -> Result<()> {
        self.item.set_inner(ItemInfo { owner: self.signer.to_account_info().key(), mint: self.mint.to_account_info().key(), lamports, bump: bumps.item });

        transfer_checked(
            CpiContext::new(self.token_program.to_account_info(), 
            TransferChecked { 
                from: self.user_ata.to_account_info(), 
                mint: self.mint.to_account_info(), 
                to: self.item_ata.to_account_info(), 
                authority: self.signer.to_account_info() 
            }), 1, 0)?;
        Ok(())
    }
}

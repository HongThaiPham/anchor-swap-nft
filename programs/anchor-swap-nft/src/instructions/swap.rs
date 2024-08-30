use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{ItemInfo, ITEM_SEED};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        mut,
        address = item.owner
    )]
    pub owner: AccountInfo<'info>,
    #[account(
       address = item.mint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [ITEM_SEED, item.owner.as_ref(), item.mint.as_ref()],
        bump
    )]
    pub item: Account<'info, ItemInfo>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = item
    )]
    pub item_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = taker
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn handler(&mut self) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.taker.to_account_info(),
                    to: self.owner.to_account_info(),
                },
            ),
            self.item.lamports,
        )?;

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
                    to: self.taker_ata.to_account_info(),
                    authority: self.item.to_account_info(),
                },
                signer_seeds,
            ),
            1,
            self.mint.decimals,
        )?;
        Ok(())
    }
}

use anchor_lang::prelude::*;

use crate::{Config, CONFIG_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump 
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner(Config { authority: self.signer.to_account_info().key(), fee, bump: bumps.config });
        Ok(())
    }
}
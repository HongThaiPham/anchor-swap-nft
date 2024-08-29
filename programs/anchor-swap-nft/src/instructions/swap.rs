use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

impl<'info> Swap<'info> {
    pub fn handler(&mut self) -> Result<()> {
        Ok(())
    }
}

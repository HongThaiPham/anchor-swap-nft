use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

impl<'info> Cancel<'info> {
    pub fn handler(&mut self) -> Result<()> {
        Ok(())
    }
}

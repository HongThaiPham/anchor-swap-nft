pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A9y6CZyMtprMecUJnjWC9fhcfrXGPGK9wwK4qxoEEj6Q");

#[program]
pub mod anchor_swap_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        ctx.accounts.handler(fee, &ctx.bumps)
    }

    pub fn create(ctx: Context<Create>, lamports: u64) -> Result<()> {
        ctx.accounts.handler(lamports, &ctx.bumps)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        ctx.accounts.handler()
    }
}

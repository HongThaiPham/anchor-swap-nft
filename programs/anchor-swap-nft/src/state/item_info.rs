use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ItemInfo {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub lamports: u64,
    pub bump: u8,
}

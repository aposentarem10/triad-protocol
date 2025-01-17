use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub ts: i64,
    pub authority: Pubkey,
    pub bump: u8,
    pub referral: Pubkey,
    pub referred: i64,
    pub name: String,
    pub swaps: i16,
    pub swaps_made: i16,
    pub staked: u64,
    pub first_swap: i64,
    pub user_trade: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserArgs {
    pub name: String,
}

impl User {
    pub const PREFIX_SEED: &'static [u8] = b"user";

    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();
}

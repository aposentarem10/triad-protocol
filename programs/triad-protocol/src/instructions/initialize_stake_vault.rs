use crate::{
    constants::ADMIN, errors::TriadProtocolError, state::InitializeStakeVaultArgs, StakeVault,
};
use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
#[instruction(args: InitializeStakeVaultArgs)]
pub struct InitializeStakeVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = StakeVault::SPACE, seeds = [StakeVault::PREFIX_SEED, args.name.as_bytes()], bump)]
    pub stake_vault: Box<Account<'info, StakeVault>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_stake_vault(
    ctx: Context<InitializeStakeVault>,
    args: InitializeStakeVaultArgs,
) -> Result<()> {
    if ctx.accounts.signer.key.to_string() != ADMIN {
        return Err(TriadProtocolError::Unauthorized.into());
    }

    let stake_vault = &mut ctx.accounts.stake_vault;

    stake_vault.bump = ctx.bumps.stake_vault;
    stake_vault.authority = *ctx.accounts.signer.key;
    stake_vault.init_ts = Clock::get()?.unix_timestamp;
    stake_vault.end_ts = Clock::get()?.unix_timestamp + 30 * 24 * 60 * 60;
    stake_vault.amount = args.amount;
    stake_vault.name = args.name;
    stake_vault.amount_paid = 0;
    stake_vault.apr = 0;
    stake_vault.amount_users = 0;
    stake_vault.collection = args.collection;
    stake_vault.slots = args.slots;
    stake_vault.is_locked = true;

    Ok(())
}

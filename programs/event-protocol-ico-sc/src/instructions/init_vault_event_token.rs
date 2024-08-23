use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};

use crate::constants::{MASTER, VAULT_TOKEN, VAULT_TOKEN_OWNER};
use crate::states::master::Master;
use crate::errors::CustomError;
use crate::InitializedVaultEventToken;

#[derive(Accounts)]
pub struct InitVaultEventTokenCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER.as_ref()],
        bump,
    )]
    master: Account<'info, Master>,

    // ===== TOKEN =====
    mint_of_event_token: Account<'info, Mint>,
    /// CHECK
    #[account(
        init, 
        payer = signer, 
        seeds = [VAULT_TOKEN_OWNER.as_ref()],
        bump,
        space = 8
    )]
    vault_owner_pda: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [VAULT_TOKEN.as_ref(), mint_of_event_token.key().as_ref()],
        token::mint = mint_of_event_token,
        token::authority = vault_owner_pda,
        bump,
    )]
    vault_event_token: Account<'info, TokenAccount>,
    // ===== =====

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program< 'info, Token>,
}

pub fn process(ctx: Context<InitVaultEventTokenCtx>) -> Result<()> {
    let master = &mut ctx.accounts.master;
    let signer = &ctx.accounts.signer;
    let clock = Clock::get()?;

    require_keys_eq!(master.owner.key(), signer.key());
    require!(!master.vault_initialized, CustomError::VaultEventTokenIsAlreadyInitialized);

    master.vault_initialized = true;

    emit!(InitializedVaultEventToken {
        from: signer.key(),
        init_time: clock.unix_timestamp as u64,
    });

    Ok(())
}

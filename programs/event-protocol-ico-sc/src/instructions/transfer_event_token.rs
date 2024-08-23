use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};

use crate::constants::{MASTER, VAULT_TOKEN, VAULT_TOKEN_OWNER};
use crate::states::master::Master;
use crate::errors::CustomError;
use crate::{DepositedEventToken, WithdrawnEventToken};

#[derive(Accounts)]
pub struct TransferEventTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [MASTER.as_ref()],
        bump,
    )]
    master: Account<'info, Master>,

    // ====== tokens =====
    mint_of_event_token: Account<'info, Mint>,
    /// CHECK
    #[account(mut,
        seeds=[VAULT_TOKEN_OWNER.as_ref()],
        bump
    )]
    vault_owner_pda: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [VAULT_TOKEN.as_ref(), mint_of_event_token.key().as_ref()],
        bump,
        token::mint = mint_of_event_token,
        token::authority = vault_owner_pda,
    )]
    vault_event_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint_of_event_token,
        token::authority = signer.key(),
    )]
    sender_event_token_ata: Account<'info, TokenAccount>,
    // ===== =====

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn deposit(ctx: Context<TransferEventTokenCtx>, amount: u64) -> Result<()> {
    let master = &ctx.accounts.master;
    let vault_event_token = &ctx.accounts.vault_event_token;
    let sender_event_token_ata = &ctx.accounts.sender_event_token_ata;
    let signer = &ctx.accounts.signer;

    require!(master.vault_initialized, CustomError::VaultEventTokenIsNotInitialized);
    require!(amount > 0, CustomError::InvalidAmount);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: sender_event_token_ata.to_account_info(),
            to: vault_event_token.to_account_info(),
            authority: signer.to_account_info()
        }
    );

    transfer(cpi_ctx, amount)?;

    emit!(DepositedEventToken {
        amount,
        from: signer.key(),
    });

    Ok(())
}

pub fn withdraw(ctx: Context<TransferEventTokenCtx>, amount: u64, is_total: bool) -> Result<()> {
    let master = &ctx.accounts.master;
    let vault_event_token = &ctx.accounts.vault_event_token;
    let sender_event_token_ata = &ctx.accounts.sender_event_token_ata;
    let vault_owner_pda = &ctx.accounts.vault_owner_pda;
    let signer = &ctx.accounts.signer;

    require_keys_eq!(master.owner.key(), signer.key(), CustomError::YouAreNotTheOwnerOfTheProgram);
    require!(master.vault_initialized, CustomError::VaultEventTokenIsNotInitialized);
    require!(amount > 0, CustomError::InvalidAmount);

    // transfer token to the user
    let bump = ctx.bumps.vault_owner_pda;
    let seeds = &[VAULT_TOKEN_OWNER.as_ref(), &[bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: vault_event_token.to_account_info(),
            to: sender_event_token_ata.to_account_info(),
            authority: vault_owner_pda.to_account_info(),
        },
        signer_seeds,
    );

    if is_total == false {
        let amount_transfer = amount * 1_000_000;
        transfer(cpi_ctx, amount_transfer)?;

        emit!(WithdrawnEventToken {
            amount,
            to: signer.key(),
        });
    } else {
        transfer(cpi_ctx, vault_event_token.amount)?;

        emit!(WithdrawnEventToken {
            amount: vault_event_token.amount,
            to: signer.key(),
        });
    }

    Ok(())
}
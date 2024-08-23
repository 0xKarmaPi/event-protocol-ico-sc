use anchor_lang::prelude::*;
// use anchor_spl::token::{Token, Mint, TokenAccount, Transfer};
// use mpl_token_metadata::accounts::Metadata;
mod events;
mod instructions;
mod states;

use events::*;
use instructions::*;

pub mod constants;
// use crate::constants::*;

pub mod errors;

declare_id!("H7PgZvp7cA8EBNkyh6TWc1rwJ1JbsZRE65fgRtmJFB6S");

#[program]
pub mod event_protocol_ico_sc {
    use super::*;

    pub fn hello(ctx: Context<HelloCtx>) -> Result<()> {
        hello::hello(ctx)
    }

    pub fn init_master(
        ctx: Context<InitMasterCtx>,
        nft_collection_address: Pubkey,
        start_sale_time: u64,
        end_sale_time: u64,
        cliff_time: u64,
        cycle: u64,
    ) -> Result<()> {
        init_master::process(
            ctx,
            nft_collection_address,
            start_sale_time,
            end_sale_time,
            cliff_time,
            cycle,
        )
    }

    pub fn init_vault_event_token(ctx: Context<InitVaultEventTokenCtx>) -> Result<()> {
        init_vault_event_token::process(ctx)
    }

    pub fn deposit_event_token(ctx: Context<TransferEventTokenCtx>, amount: u64) -> Result<()> {
        transfer_event_token::deposit(ctx, amount)
    }

    pub fn withdraw_event_token(
        ctx: Context<TransferEventTokenCtx>,
        amount: u64,
        is_total: bool,
    ) -> Result<()> {
        transfer_event_token::withdraw(ctx, amount, is_total)
    }

    pub fn init_a_wrapper(ctx: Context<InitAWrapperCtx>, nft_address: Pubkey) -> Result<()> {
        init_a_wrapper::process(ctx, nft_address)
    }

    pub fn claim_token(ctx: Context<ClaimTokenCtx>, nft_address: Pubkey) -> Result<()> {
        claim_token::process(ctx, nft_address)
    }

    // update state
    pub fn update_start_sale_time(
        ctx: Context<UpdateMasterCtx>,
        start_sale_time: u64,
    ) -> Result<()> {
        ctx.accounts.update_start_sale_time(start_sale_time)
    }

    pub fn update_end_sale_time(ctx: Context<UpdateMasterCtx>, end_sale_time: u64) -> Result<()> {
        ctx.accounts.update_end_sale_time(end_sale_time)
    }

    pub fn update_cliff_time(ctx: Context<UpdateMasterCtx>, cliff_time: u64) -> Result<()> {
        ctx.accounts.update_cliff_time(cliff_time)
    }

    pub fn update_cycle(ctx: Context<UpdateMasterCtx>, cycle: u64) -> Result<()> {
        ctx.accounts.update_cycle(cycle)
    }
}

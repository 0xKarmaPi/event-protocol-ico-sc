use anchor_lang::prelude::*;

use crate::constants::MASTER;
use crate::errors::CustomError;
use crate::states::master::Master;
use crate::InitializedMaster;

#[derive(Accounts)]
pub struct InitMasterCtx<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [MASTER.as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Master>(),
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<InitMasterCtx>,
    nft_collection_address: Pubkey,
    start_sale_time: u64,
    end_sale_time: u64,
    cliff_time: u64,
    cycle: u64,
) -> Result<()> {
    let master = &mut ctx.accounts.master;
    let signer = &ctx.accounts.signer;
    let clock = Clock::get()?;

    require!(!master.initialized, CustomError::MasterIsAlreadyInitialized);
    require!(start_sale_time < end_sale_time, CustomError::InvalidTimeRange);
    require!(cliff_time > 0, CustomError::InvalidCliffTime);
    require!(cycle > 0, CustomError::InvalidCycle);

    master.initialized = true;
    master.owner = signer.key();
    master.vault_initialized = false;
    master.nft_collection_address = nft_collection_address;
    master.start_sale_time = start_sale_time;
    master.end_sale_time = end_sale_time;
    master.cliff_time = cliff_time;
    master.cycle = cycle;
    master.total_wrapper = 2000;
    master.total_sold_wrapper = 0;
    master.token_per_wrapper = 5000;

    emit!(InitializedMaster {
        from: signer.key(),
        initialized: true,
        init_time: clock.unix_timestamp as u64,
        nft_collection_address,
        start_sale_time,
        total_wrapper: 2000,
    });

    Ok(())
}

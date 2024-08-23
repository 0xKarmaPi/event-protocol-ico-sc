use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use mpl_token_metadata::accounts::Metadata;

use crate::constants::{MASTER, WRAPPER};
use crate::states::master::Master;
use crate::states::wrapper::Wrapper;
use crate::errors::CustomError;
use crate::InitializedAPackge;

#[derive(Accounts)]
#[instruction(nft_address: Pubkey)]
pub struct InitAWrapperCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER.as_ref()],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [WRAPPER.as_ref(), nft_address.as_ref()],
        bump, 
        space = 8 + std::mem::size_of::<Wrapper>(),
    )]
    wrapper: Account<'info, Wrapper>,

    // ===== NFT
    mint_of_nft: Account<'info, Mint>,
    /// CHECK
    #[account(mut)]
    mint_nft_metadata_account: AccountInfo<'info>,
    #[account(
        mut, 
        token::mint = mint_of_nft,
        token::authority = signer,
        constraint = sender_nft_ata.owner == signer.key(),
        constraint = sender_nft_ata.amount == 1 @ CustomError::RequiredAnNFT,
    )]
    sender_nft_ata: Account<'info, TokenAccount>,
    // =====

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<InitAWrapperCtx>, nft_address: Pubkey) -> Result<()> {
    let clock = Clock::get()?;
    let wrapper = &mut ctx.accounts.wrapper;
    let master = &mut ctx.accounts.master;
    let signer = &ctx.accounts.signer;

    // check the collection of the NFT
    let metadata_account = &ctx.accounts.mint_nft_metadata_account;
    let metadata_data: &[u8] = &metadata_account.data.borrow();
    let nft_metadata: Metadata = Metadata::from_bytes(metadata_data)
    .map_err(|_| ProgramError::InvalidAccountData)?;

    // check if the total sold wrapper is exceeded
    if master.total_sold_wrapper > master.total_wrapper {
        return Err(CustomError::TotalWrapperIsExceeded.into());
    } 

    if (clock.unix_timestamp as u64) < master.start_sale_time || (clock.unix_timestamp as u64) > master.end_sale_time {
        return Err(CustomError::InvalidTimeRange.into());
    }

    // check if the NFT is from the correct collection or not
    if let Some(collection) = &nft_metadata.collection {
        if collection.key != master.nft_collection_address {
            msg!("Collection Key is not valid: {}", collection.key);
            return Err(CustomError::CollectionKeyIsNotValid.into());
        }
    } else {
        return Err(CustomError::CollectionKeyIsNotValid.into());
    }

    wrapper.initialized = true;
    wrapper.nft_address = nft_address;
    wrapper.init_time = clock.unix_timestamp as u64;
    wrapper.start_time = clock.unix_timestamp as u64 + master.cliff_time;
    wrapper.amount_of_tokens_claimed = 0;

    master.total_sold_wrapper += 1;

    emit!(InitializedAPackge {
        from: signer.key(),
        nft_address,
        init_time: clock.unix_timestamp as u64,
    });

    Ok(())
}
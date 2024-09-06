use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};
use mpl_token_metadata::accounts::Metadata;

use crate::constants::{MASTER, VAULT_TOKEN, VAULT_TOKEN_OWNER, WRAPPER};
use crate::states::{master::Master, wrapper::Wrapper};
use crate::errors::CustomError;
use crate::ClaimedEventToken;

#[derive(Accounts)]
#[instruction(nft_address: Pubkey)]
pub struct ClaimTokenCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER.as_ref()],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        mut,
        seeds = [WRAPPER.as_ref(), nft_address.as_ref()],
        bump, 
    )]
    wrapper: Account<'info, Wrapper>,

    mint_of_nft: Account<'info, Mint>,
    #[account(
        mut, 
        token::mint = mint_of_nft,
        token::authority = signer,
        constraint = sender_nft_ata.owner == signer.key(),
        constraint = sender_nft_ata.amount == 1 @ CustomError::RequiredAnNFT,
    )]
    sender_nft_ata: Account<'info, TokenAccount>,
    /// CHECK
    #[account(mut)]
    mint_metadata_account: AccountInfo<'info>,

    // tokens
    mint_of_event_token: Account<'info, Mint>,
    /// CHECK
    #[account(
        mut, 
        seeds = [VAULT_TOKEN_OWNER.as_ref()],
        bump,
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
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_of_event_token, 
        associated_token::authority = signer,
    )]
    sender_event_token_ata: Account<'info, TokenAccount>,
    // ======

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process(ctx: Context<ClaimTokenCtx>, nft_address: Pubkey) -> Result<()> {
    let master = &ctx.accounts.master;
    let wrapper = &mut ctx.accounts.wrapper;
    let mint_of_event_token = &ctx.accounts.mint_of_event_token;
    let vault_owner_pda = &ctx.accounts.vault_owner_pda;
    let vault_event_token = &ctx.accounts.vault_event_token;
    let sender_event_token_ata = &ctx.accounts.sender_event_token_ata;
    let signer = &ctx.accounts.signer;
    let clock = Clock::get()?;

    // check the collection of the NFT
    let metadata_account = &ctx.accounts.mint_metadata_account;
    let metadata_data: &[u8] = &metadata_account.data.borrow();
    let nft_metadata: Metadata = Metadata::from_bytes(metadata_data)
    .map_err(|_| ProgramError::InvalidAccountData)?;

    // local variables
    let current_time = clock.unix_timestamp;

    // check if wrapper is not initialized
    require!(wrapper.initialized, CustomError::WrapperNotInitialized);
    // check time
    require!(current_time as u64 > wrapper.init_time + master.cliff_time, CustomError::CliffTimeNotReachedYet);
    // 
    require!(wrapper.amount_of_tokens_claimed < master.token_per_wrapper, CustomError::ClaimedAllTokens);

    // check if the NFT is from the correct collection or not
    if let Some(collection) = &nft_metadata.collection {
        let collection_key = collection.key;
        if collection_key != master.nft_collection_address {
            msg!("Collection Key is not valid");
            return Err(CustomError::CollectionKeyIsNotValid.into());
        }
    } else {
        return Err(CustomError::CollectionKeyIsNotValid.into());
    }

    // ======
    // Calculate the number of cycles (periods) that have passed since cliff time
    let time_since_cliff = current_time as u64 - (wrapper.init_time + master.cliff_time);
    // claim 10% of the tokens per cycle
    let cycles_elapsed = ((time_since_cliff / master.cycle) + 1).min(10); // +1 for the initial claim after cliff

    msg!("cycles_elapsed: {}", cycles_elapsed);

    let amount_max_claimable = (master.token_per_wrapper * 10 * cycles_elapsed as u64) / 100;
    let amount_claimable_tokens = (amount_max_claimable.saturating_sub(wrapper.amount_of_tokens_claimed)).min(master.token_per_wrapper - wrapper.amount_of_tokens_claimed);

    msg!("amount_max_claimable: {}", amount_max_claimable);
    msg!("amount_claimable_tokens: {}", amount_claimable_tokens);
    msg!("amount_claimed_tokens: {}", wrapper.amount_of_tokens_claimed);

    // transfer token
    require!(amount_claimable_tokens > 0, CustomError::NoClaimableAmount);

    // update the wrapper
    wrapper.amount_of_tokens_claimed += amount_claimable_tokens;
    wrapper.last_claimed_time = clock.unix_timestamp as u64;
    msg!("AFTER amount_claimed_tokens: {}", wrapper.amount_of_tokens_claimed);

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

    transfer(cpi_ctx, amount_claimable_tokens * (10u64.pow(mint_of_event_token.decimals as u32)))?;

    emit!(ClaimedEventToken {
        from: signer.key(),
        nft_address,
        claim_time: clock.unix_timestamp as u64,
        claimed_amount: amount_claimable_tokens,
    });

    Ok(())
}

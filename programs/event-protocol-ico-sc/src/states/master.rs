use anchor_lang::prelude::*;

#[account]
pub struct Master {
    pub initialized: bool,
    pub owner: Pubkey,
    pub vault_initialized: bool,
    pub nft_collection_address: Pubkey,
    pub start_sale_time: u64, // in secs
    pub end_sale_time: u64, // in secs
    pub cliff_time: u64, // in secs - 60 days * 24 hours * 60 mins * 60 secs
    pub cycle: u64, // in secs - 30 days * 24 hours * 60 mins * 60 secs
    pub total_wrapper: u64,
    pub total_sold_wrapper: u64,
    pub token_per_wrapper: u64,
}

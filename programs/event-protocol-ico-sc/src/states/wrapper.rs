use anchor_lang::prelude::*;

#[account]
pub struct Wrapper {
    pub initialized: bool,
    pub nft_address: Pubkey,
    pub init_time: u64, // open time
    pub start_time: u64, // start claim time
    pub amount_of_tokens_claimed: u64,
    pub last_claimed_time: u64,
}
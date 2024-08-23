use anchor_lang::prelude::*;

#[event]
pub struct InitializedMaster {
    pub from: Pubkey,
    pub initialized: bool,
    pub init_time: u64,
    pub nft_collection_address: Pubkey,
    pub start_sale_time: u64,
    pub total_wrapper: u64,
}

#[event]
pub struct InitializedVaultEventToken {
    pub from: Pubkey,
    pub init_time: u64,
}

#[event]
pub struct DepositedEventToken {
    pub amount: u64,
    pub from: Pubkey,
}

#[event]
pub struct InitializedAPackge {
    pub from: Pubkey,
    pub nft_address: Pubkey,
    pub init_time: u64,
}

#[event]
pub struct ClaimedEventToken {
    pub from: Pubkey,
    pub nft_address: Pubkey,
    pub claim_time: u64,
    pub claimed_amount: u64,
}

#[event]
pub struct WithdrawnEventToken {
    pub amount: u64,
    pub to: Pubkey,
}

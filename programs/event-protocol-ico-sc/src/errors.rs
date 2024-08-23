use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    // init master
    #[msg("Master is already initialized")]
    MasterIsAlreadyInitialized,
    #[msg("Invalid time range")]
    InvalidTimeRange,
    #[msg("Invalid cliff time")]
    InvalidCliffTime,
    #[msg("Invalid cycle")]
    InvalidCycle,
    //
    #[msg("Vault EVENT token is already initialized")]
    VaultEventTokenIsAlreadyInitialized,
    #[msg("Vault EVENT token is not initialized")]
    VaultEventTokenIsNotInitialized,
    #[msg("Invalid amount")]
    InvalidAmount,
    //
    #[msg("Cliff time not reached yet")]
    CliffTimeNotReachedYet,
    #[msg("Wrapper not initialized")]
    WrapperNotInitialized,
    #[msg("wrapper_id is not valid")]
    WrapperIdIsNotValid,
    #[msg("Collection key is not valid")]
    CollectionKeyIsNotValid,
    #[msg("Wrapper already initialized")]
    WrapperAlreadyInitialized,
    #[msg("You are not the owner of the NFT")]
    YouAreNotTheOwnerOfTheNFT,
    #[msg("Required an NFT")]
    RequiredAnNFT,
    #[msg("No claimable amount")]
    NoClaimableAmount,
    #[msg("Claimed all tokens")]
    ClaimedAllTokens,
    
    #[msg("You are not the owner of the program")]
    YouAreNotTheOwnerOfTheProgram,

    #[msg("Total wrapper is exceeded")]
    TotalWrapperIsExceeded,
}

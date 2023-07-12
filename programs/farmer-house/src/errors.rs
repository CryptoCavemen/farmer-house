use anchor_lang::prelude::*;

#[error_code]
pub enum FarmerHouseError {
    #[msg("A required signature is missing")]
    MissingRequiredSignature,
    #[msg("The owner does not match")]
    OwnerMismatch,
    #[msg("The program ID does not match")]
    ProgramMismatch,
    #[msg("The authority does not match")]
    AuthorityMismatch,
    #[msg("The mint address does not match")]
    MintMismatch,
    #[msg("The source ATA does not have enough tokens to transfer")]
    AmountMismatch,
    #[msg("The collection does not match, is unnauthorised or is null")]
    CollectionMismatch,
    #[msg("The collection is not verified")]
    CollectionNotVerified,
    #[msg("The trifle account does not match or is null")]
    TrifleMismatch,
    #[msg("The crop is not ready to be harvested or sold, you have to water it a bit more")]
    CropNotRipe,
    #[msg("The crop is ready to be harvested and does not need to be watered anymore")]
    CropReady,
}
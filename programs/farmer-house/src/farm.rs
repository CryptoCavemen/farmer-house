use {
    crate::{errors::*, program::FarmerHouse as program_farmer_house, FarmsPda},
    anchor_lang::{
        prelude::*,
        solana_program::program::{invoke, invoke_signed},
    },
    anchor_spl::{associated_token, token},
    mpl_token_metadata::{
        instruction::update_metadata_accounts_v2,
        state::{Collection, DataV2, Metadata, TokenMetadataAccount},
        utils::assert_derivation,
        ID as TOKEN_METADATA_ID,
    },
    mpl_trifle::{
        instruction as trifle_instruction,
        state::{trifle::Trifle, SolanaAccount, TRIFLE_SEED},
        ID as TRIFLE_PROGRAM_ID,
    },
};

pub fn plant_seed(ctx: Context<PlantSeed>, crop_slot: String) -> Result<()> {
    // Check for all program accounts (in case they're not being correctly checked on trifle)
    require_eq!(
        ctx.accounts.token_program.key(),
        token::ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.associated_token_program.key(),
        associated_token::ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.token_metadata_program.key(),
        TOKEN_METADATA_ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.trifle_program.key(),
        TRIFLE_PROGRAM_ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.farmer_house_program.key(),
        program_farmer_house::id(),
        FarmerHouseError::ProgramMismatch
    );

    let metadata_info =
        Metadata::from_account_info(&ctx.accounts.attribute_metadata.to_account_info());
    require_eq!(
        ctx.accounts.farms_pda.tomato_seed_collection_address,
        metadata_info
            .as_ref()
            .unwrap()
            .collection
            .as_ref()
            .unwrap()
            .key,
        FarmerHouseError::CollectionMismatch
    );

    // We don't verify collection, in a real case scenario it would be important to have it verified and check if it is so

    invoke(
        &trifle_instruction::transfer_in(
            TRIFLE_PROGRAM_ID,
            ctx.accounts.trifle_account.key(),
            ctx.accounts.farms_pda.key(),
            ctx.accounts.user_account.key(),
            ctx.accounts.escrow_constraint_model.key(),
            ctx.accounts.escrow_account.key(),
            Some(ctx.accounts.field_mint.key()),
            Some(ctx.accounts.field_token_account.key()),
            Some(ctx.accounts.field_master_edition.key()),
            ctx.accounts.attribute_mint.key(),
            ctx.accounts.attribute_src_token_account.key(),
            Some(ctx.accounts.attribute_dst_token_account.key()),
            Some(ctx.accounts.attribute_metadata.key()),
            None,
            None,
            crop_slot,
            1,
        ),
        &[
            ctx.accounts.trifle_account.to_account_info(),
            ctx.accounts.farms_pda.to_account_info(),
            ctx.accounts.user_account.to_account_info(),
            ctx.accounts.escrow_constraint_model.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
            ctx.accounts.field_mint.to_account_info(),
            ctx.accounts.field_token_account.to_account_info(),
            ctx.accounts.field_master_edition.to_account_info(),
            ctx.accounts.attribute_mint.to_account_info(),
            ctx.accounts.attribute_src_token_account.to_account_info(),
            ctx.accounts.attribute_dst_token_account.to_account_info(),
            ctx.accounts.attribute_metadata.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.trifle_program.to_account_info(),
        ],
    )?;

    Ok(())
}

pub fn water(ctx: Context<Water>, crop_slot: String) -> Result<()> {
    let farms_pda_info = &ctx.accounts.farms_pda;
    let metadata_info = Metadata::from_account_info(&ctx.accounts.crop_metadata.to_account_info());

    assert_ne!(
        metadata_info
            .as_ref()
            .unwrap()
            .collection
            .as_ref()
            .unwrap()
            .key,
        farms_pda_info.ripe_tomato_collection_address,
        "{}",
        FarmerHouseError::CropReady
    );

    assert_eq!(
        ctx.accounts.token_metadata_program.key(),
        TOKEN_METADATA_ID,
        "{}",
        FarmerHouseError::ProgramMismatch
    );

    // Confirm field_mint is the mint in field_associated_token
    assert_eq!(
        ctx.accounts.field_associated_token.mint,
        ctx.accounts.field_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );

    // Confirm user is owner of field_associated_account
    assert_eq!(
        ctx.accounts.field_associated_token.owner,
        ctx.accounts.user_account.key(),
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    assert!(
        !ctx.accounts.trifle_account.data_is_empty(),
        "{}",
        FarmerHouseError::TrifleMismatch
    );

    assert_derivation(
        &TRIFLE_PROGRAM_ID,
        &ctx.accounts.trifle_account,
        &[
            TRIFLE_SEED.as_bytes(),
            ctx.accounts.field_mint.key().as_ref(),
            ctx.accounts.farms_pda.key().as_ref(),
        ],
    )?;

    let trifle = Trifle::from_account_info(&ctx.accounts.trifle_account.to_account_info())?;
    assert!(
        trifle
            .tokens
            .get(&crop_slot)
            .unwrap()
            .get(0)
            .unwrap()
            .amount
            == 1,
        "{}",
        FarmerHouseError::TrifleMismatch
    );

    let data: DataV2;

    if metadata_info
        .as_ref()
        .unwrap()
        .collection
        .as_ref()
        .unwrap()
        .key
        == farms_pda_info.tomato_seed_collection_address
    {
        data = DataV2 {
            name: String::from("Tomato Sapling"),
            symbol: String::from("TOMATO"),
            uri: String::from(
                "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
            ),
            seller_fee_basis_points: 1,
            creators: None,
            collection: Some(Collection {
                verified: (false),
                key: (farms_pda_info.tomato_sappling_collection_address),
            }),
            uses: None,
        };
    } else if metadata_info
        .as_ref()
        .unwrap()
        .collection
        .as_ref()
        .unwrap()
        .key
        == farms_pda_info.tomato_sappling_collection_address
    {
        data = DataV2 {
            name: String::from("Ripe Tomato"),
            symbol: String::from("TOMATO"),
            uri: String::from(
                "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
            ),
            seller_fee_basis_points: 1,
            creators: None,
            collection: Some(Collection {
                verified: (false),
                key: (farms_pda_info.ripe_tomato_collection_address),
            }),
            uses: None,
        };
    } else {
        return err!(FarmerHouseError::CollectionMismatch);
    };

    invoke_signed(
        &update_metadata_accounts_v2(
            TOKEN_METADATA_ID,
            ctx.accounts.crop_metadata.key(),
            ctx.accounts.farms_pda.key(),
            None,
            Some(data),
            None,
            None,
        ),
        &[
            ctx.accounts.crop_metadata.to_account_info(),
            ctx.accounts.farms_pda.to_account_info(),
        ],
        &[&[
            b"farmer-house-farms",
            ctx.accounts.farmer_house_program.key().as_ref(),
            &[ctx.accounts.farms_pda.bump],
        ]],
    )?;

    Ok(())
}

pub fn harvest_crop(ctx: Context<HarvestCrop>, crop_slot: String) -> Result<()> {
     // Check for all program accounts (in case they're not being correctly checked on trifle)
     require_eq!(
        ctx.accounts.token_program.key(),
        token::ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.associated_token_program.key(),
        associated_token::ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.token_metadata_program.key(),
        TOKEN_METADATA_ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.trifle_program.key(),
        TRIFLE_PROGRAM_ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.farmer_house_program.key(),
        program_farmer_house::id(),
        FarmerHouseError::ProgramMismatch
    );

    let metadata_info =
        Metadata::from_account_info(&ctx.accounts.attribute_metadata.to_account_info());
    require_eq!(
        ctx.accounts.farms_pda.ripe_tomato_collection_address,
        metadata_info
            .as_ref()
            .unwrap()
            .collection
            .as_ref()
            .unwrap()
            .key,
        FarmerHouseError::CollectionMismatch
    );

    invoke(
        &trifle_instruction::transfer_out(
            TRIFLE_PROGRAM_ID,
            ctx.accounts.trifle_account.key(),
            ctx.accounts.escrow_constraint_model.key(),
            ctx.accounts.escrow_account.key(),
            ctx.accounts.field_token_account.key(),
            ctx.accounts.field_mint.key(),
            ctx.accounts.field_metadata.key(),
            Some(ctx.accounts.field_edition.key()),
            ctx.accounts.user_account.key(),
            ctx.accounts.farms_pda.key(),
            ctx.accounts.attribute_mint.key(),
            ctx.accounts.attribute_src_token_account.key(),
            ctx.accounts.attribute_dst_token_account.key(),
            ctx.accounts.attribute_metadata.key(),
            crop_slot,
            1,
        ),
        &[
            ctx.accounts.trifle_account.to_account_info(),
            ctx.accounts.escrow_constraint_model.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
            ctx.accounts.field_token_account.to_account_info(),
            ctx.accounts.field_mint.to_account_info(),
            ctx.accounts.field_metadata.to_account_info(),
            ctx.accounts.field_edition.to_account_info(),
            ctx.accounts.user_account.to_account_info(),
            ctx.accounts.farms_pda.to_account_info(),
            ctx.accounts.attribute_mint.to_account_info(),
            ctx.accounts.attribute_src_token_account.to_account_info(),
            ctx.accounts.attribute_dst_token_account.to_account_info(),
            ctx.accounts.attribute_metadata.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.trifle_program.to_account_info(),
            ctx.accounts.instructions.to_account_info(),
        ],
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct PlantSeed<'info> {
    /// CHECK: Trifle will check this
    #[account(mut)]
    pub trifle_account: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Box<Account<'info, FarmsPda>>,
    #[account(mut)]
    pub user_account: Signer<'info>,
    /// CHECK: Trifle will check this
    #[account(mut)]
    pub escrow_constraint_model: UncheckedAccount<'info>,

    /// CHECK: 
    pub escrow_account: UncheckedAccount<'info>,
    /// CHECK: 
    #[account(mut)]
    pub field_mint: Box<Account<'info, token::Mint>>,
    /// CHECK: 
    #[account(mut)]
    pub field_token_account: UncheckedAccount<'info>,
    /// CHECK: 
    #[account(mut)]
    pub field_master_edition: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub attribute_mint: Box<Account<'info, token::Mint>>,
    /// CHECK:
    #[account(mut)]
    pub attribute_src_token_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub attribute_dst_token_account: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub attribute_metadata: UncheckedAccount<'info>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Trifle will check this
    pub trifle_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Water<'info> {
    #[account(seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Account<'info, FarmsPda>,

    #[account(mut)]
    pub user_account: Signer<'info>,

    #[account(mut)]
    pub field_mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub field_associated_token: Box<Account<'info, token::TokenAccount>>,

    /// CHECK:
    #[account(mut)]
    pub crop_metadata: UncheckedAccount<'info>,

    /// CHECK: Trifle will check this
    pub trifle_account: UncheckedAccount<'info>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HarvestCrop<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub trifle_account: UncheckedAccount<'info>,
    #[account(seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Box<Account<'info, FarmsPda>>,
    #[account(mut)]
    pub user_account: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub escrow_constraint_model: UncheckedAccount<'info>,

    /// CHECK: We love pain.
    pub escrow_account: UncheckedAccount<'info>,
    /// CHECK: We love pain.
    #[account(mut)]
    pub field_mint: Box<Account<'info, token::Mint>>,
    /// CHECK: We love pain.
    #[account(mut)]
    pub field_token_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK: We love pain.
    #[account(mut)]
    pub field_metadata: UncheckedAccount<'info>,
    /// CHECK: We love pain.
    #[account(mut)]
    pub field_edition: UncheckedAccount<'info>,

    /// CHECK:
    pub attribute_mint: Box<Account<'info, token::Mint>>,
    /// CHECK:
    #[account(mut)]
    pub attribute_src_token_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub attribute_dst_token_account: Box<Account<'info, token::TokenAccount>>,
    /// CHECK:
    pub attribute_metadata: UncheckedAccount<'info>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Trifle will check this
    pub trifle_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub instructions: UncheckedAccount<'info>,
}

use mpl_token_metadata::assertions::collection::assert_master_edition;

use {
    crate::{errors::*, program::FarmerHouse as program_farmer_house, FarmsPda},
    anchor_lang::{
        prelude::*,
        solana_program::{program::invoke_signed, sysvar::instructions::ID as INSTRUCTIONS_ID},
    },
    anchor_spl::{associated_token, token},
    mpl_token_metadata::{state::{Metadata, TokenMetadataAccount}, ID as TOKEN_METADATA_ID},
    mpl_trifle::{instruction as trifle_instruction, ID as TRIFLE_PROGRAM_ID},
};

pub fn buy_field(ctx: Context<BuyField>) -> Result<()> {
    let farms_pda_info = &ctx.accounts.farms_pda;

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

    require_eq!(
        ctx.accounts.instructions.key(),
        INSTRUCTIONS_ID,
        FarmerHouseError::ProgramMismatch
    );

    // Confirm token_program is owner of associated token accounts and mint accounts
    assert_eq!(
        ctx.accounts.spl_ata_source.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts
            .spl_ata_destination
            .as_ref()
            .to_account_info()
            .owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts.field_ata_source.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    assert_eq!(
        ctx.accounts.spl_mint.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts.field_mint.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts.field_master_edition.as_ref().to_account_info().owner,
        &TOKEN_METADATA_ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    assert_eq!(
        ctx.accounts.field_metadata.as_ref().to_account_info().owner,
        &TOKEN_METADATA_ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    assert_eq!(
        ctx.accounts.escrow_constraint_model.as_ref().to_account_info().owner,
        &TRIFLE_PROGRAM_ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    let binding = Metadata::from_account_info(&ctx.accounts.field_metadata.to_account_info());
    let metadata_info = binding.as_ref().unwrap();

    assert_eq!(
        metadata_info
            .collection
            .as_ref()
            .unwrap()
            .key,
        farms_pda_info.field_collection_address,
        "{}",
        FarmerHouseError::CollectionMismatch
    );
    assert_eq!(
        metadata_info.mint,
        ctx.accounts.field_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_master_edition(metadata_info, &ctx.accounts.field_master_edition)?;

    // Assert spl_mint and it's associated accounts are correctly set up
    assert_eq!(
        ctx.accounts.spl_mint.key(),
        farms_pda_info.spl_mint_address,
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_source.mint,
        ctx.accounts.spl_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_destination.mint,
        ctx.accounts.spl_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );

    assert_eq!(
        ctx.accounts.field_ata_source.mint,
        ctx.accounts.field_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );

    // Assert ATA sources have enough tokens to transfer out
    assert!(
        ctx.accounts.spl_ata_source.amount >= 70000000,
        "{}",
        FarmerHouseError::AmountMismatch
    );
    assert!(
        ctx.accounts.field_ata_source.amount >= 1,
        "{}",
        FarmerHouseError::AmountMismatch
    );

    // No need to check associated_token (field_ata_destination) against authority (user_account)
    // because it will be checked in this next step,
    // therefore if the check does not pass the program will not incur any costs besided the tx fee

    if *ctx.accounts.field_ata_destination.owner != token::spl_token::ID
        && ctx.accounts.field_ata_destination.lamports() == 0
    {
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.user_account.to_account_info(),
                associated_token: ctx.accounts.field_ata_destination.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
                mint: ctx.accounts.field_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;
    }

    // Transfer spl token from ATA of user to ATA of pda
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.spl_ata_source.to_account_info(),
                to: ctx.accounts.spl_ata_destination.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
            },
        ),
        70000000,
    )?;

    // Transfer fieldNFT from ATA of FarmsPda to ATA of user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.field_ata_source.to_account_info(),
                to: ctx.accounts.field_ata_destination.to_account_info(),
                authority: ctx.accounts.farms_pda.to_account_info(),
            },
            &[&[
                b"farmer-house-farms",
                ctx.accounts.farmer_house_program.key().as_ref(),
                &[ctx.accounts.farms_pda.bump],
            ]],
        ),
        1,
    )?;

    if *ctx.accounts.trifle_account.owner != TRIFLE_PROGRAM_ID
        && ctx.accounts.trifle_account.lamports() == 0
    {
        invoke_signed(
            &trifle_instruction::create_trifle_account(
                &TRIFLE_PROGRAM_ID,
                &ctx.accounts.escrow_account.key(),
                &ctx.accounts.field_metadata.key(),
                &ctx.accounts.field_mint.key(),
                &ctx.accounts.field_ata_destination.key(),
                &ctx.accounts.field_master_edition.key(),
                &ctx.accounts.trifle_account.key(),
                &ctx.accounts.farms_pda.key(),
                &ctx.accounts.escrow_constraint_model.key(),
                &ctx.accounts.user_account.key(),
            ),
            &[
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.field_metadata.to_account_info(),
                ctx.accounts.field_mint.to_account_info(),
                ctx.accounts.field_ata_destination.to_account_info(),
                ctx.accounts.field_master_edition.to_account_info(),
                ctx.accounts.trifle_account.to_account_info(),
                ctx.accounts.farms_pda.to_account_info(),
                ctx.accounts.escrow_constraint_model.to_account_info(),
                ctx.accounts.user_account.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.trifle_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.instructions.to_account_info(),
            ],
            &[&[
                b"farmer-house-farms",
                ctx.accounts.farmer_house_program.key().as_ref(),
                &[ctx.accounts.farms_pda.bump],
            ]],
        )?;
    }

    Ok(())
}

pub fn buy_seed(ctx: Context<BuySeed>) -> Result<()> {
    let farms_pda_info = &ctx.accounts.farms_pda;

    // Confirm farms_pda is owned by program
    assert_eq!(
        ctx.accounts.farms_pda.to_account_info().as_ref().owner,
        &program_farmer_house::id(),
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    // Confirm token_program is owner of associated token accounts
    assert_eq!(
        ctx.accounts.spl_ata_source.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts
            .spl_ata_destination
            .as_ref()
            .to_account_info()
            .owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts.seed_ata_source.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    assert_eq!(
        ctx.accounts.spl_mint.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts.seed_mint.as_ref().to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    let binding = Metadata::from_account_info(&ctx.accounts.seed_metadata.to_account_info());
    let metadata_info = binding.as_ref().unwrap();

    assert_eq!(
        metadata_info
            .collection
            .as_ref()
            .unwrap()
            .key,
        farms_pda_info.tomato_seed_collection_address,
        "{}",
        FarmerHouseError::CollectionMismatch
    );
    assert_eq!(
        metadata_info.mint,
        ctx.accounts.seed_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );

    //Assert spl_mint and it's associated accounts are correctly set up
    assert_eq!(
        ctx.accounts.spl_mint.key(),
        farms_pda_info.spl_mint_address,
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_source.mint,
        farms_pda_info.spl_mint_address,
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_destination.mint,
        farms_pda_info.spl_mint_address,
        "{}",
        FarmerHouseError::MintMismatch
    );

    assert_eq!(
        ctx.accounts.seed_ata_source.mint,
        ctx.accounts.seed_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );
    
    // Assert ATA sources have enough tokens to transfer out
    assert!(
        ctx.accounts.spl_ata_source.amount >= 2000000,
        "{}",
        FarmerHouseError::AmountMismatch
    );
    assert!(
        ctx.accounts.seed_ata_source.amount >= 1,
        "{}",
        FarmerHouseError::AmountMismatch
    );

    // No need to check associated_token (field_ata_destination) against authority (user_account)
    // because it will be checked in this next step,
    // therefore if the check does not pass the program will not incur any costs besided the tx fee

    if *ctx.accounts.seed_ata_destination.owner != token::spl_token::ID
        && ctx.accounts.seed_ata_destination.lamports() == 0
    {
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.user_account.to_account_info(),
                associated_token: ctx.accounts.seed_ata_destination.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
                mint: ctx.accounts.seed_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;
    }

    // Transfer spl token from ATA of user to ATA of pda
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.spl_ata_source.to_account_info(),
                to: ctx.accounts.spl_ata_destination.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
            },
        ),
        2000000,
    )?;

    // Transfer seedNFT from ATA of FarmsPda to ATA of user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.seed_ata_source.to_account_info(),
                to: ctx.accounts.seed_ata_destination.to_account_info(),
                authority: ctx.accounts.farms_pda.to_account_info(),
            },
            &[&[
                b"farmer-house-farms",
                ctx.accounts.farmer_house_program.key().as_ref(),
                &[ctx.accounts.farms_pda.bump],
            ]],
        ),
        1,
    )?;

    Ok(())
}

pub fn sell_crop(ctx: Context<SellCrop>) -> Result<()> {
    let farms_pda_info = &ctx.accounts.farms_pda;
    let metadata_info = Metadata::from_account_info(&ctx.accounts.crop_metadata.to_account_info());

    // Confirm token_program is owner of associated token accounts
    assert_eq!(
        ctx.accounts.spl_ata_source.to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_destination.to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    // Confirm token_program is owner of mint account
    assert_eq!(
        ctx.accounts.spl_mint.to_account_info().owner,
        &token::ID,
        "{}",
        FarmerHouseError::OwnerMismatch
    );

    //Assert spl_mint and it's associated accounts are correctly set up
    assert_eq!(
        ctx.accounts.spl_mint.key(),
        farms_pda_info.spl_mint_address,
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_source.mint,
        ctx.accounts.spl_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );
    assert_eq!(
        ctx.accounts.spl_ata_destination.mint,
        ctx.accounts.spl_mint.key(),
        "{}",
        FarmerHouseError::MintMismatch
    );

    assert_eq!(
        &metadata_info.as_ref().unwrap().mint,
        &ctx.accounts.crop_ata_source.mint,
        "{}",
        FarmerHouseError::MintMismatch
    );

    assert_eq!(
        &metadata_info.as_ref().unwrap().mint,
        &ctx.accounts.crop_ata_destination.mint,
        "{}",
        FarmerHouseError::MintMismatch
    );

    let price: u64;

    if metadata_info
        .as_ref()
        .unwrap()
        .collection
        .as_ref()
        .unwrap()
        .key
        == farms_pda_info.tomato_sappling_collection_address
    {
        price = 3500000;
    } else if metadata_info
        .as_ref()
        .unwrap()
        .collection
        .as_ref()
        .unwrap()
        .key
        == farms_pda_info.ripe_tomato_collection_address
    {
        price = 7000000;
    } else {
        price = 2000000;
    }
    
    // Assert ATA sources have enough tokens to transfer out
    assert!(
        ctx.accounts.spl_ata_source.amount >= price,
        "{}",
        FarmerHouseError::AmountMismatch
    );
    assert!(
        ctx.accounts.crop_ata_source.amount >= 1,
        "{}",
        FarmerHouseError::AmountMismatch
    );

    // Transfer cropNFT from ATA of FarmsPda to ATA of user
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.crop_ata_source.to_account_info(),
                to: ctx.accounts.crop_ata_destination.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
            },
        ),
        1,
    )?;

    // Transfer spl token from ATA of farmsPda to ATA of user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.spl_ata_source.to_account_info(),
                to: ctx.accounts.spl_ata_destination.to_account_info(),
                authority: ctx.accounts.farms_pda.to_account_info(),
            },
            &[&[
                b"farmer-house-farms",
                ctx.accounts.farmer_house_program.key().as_ref(),
                &[ctx.accounts.farms_pda.bump],
            ]],
        ),
        price,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct BuyField<'info> {
    #[account(seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Account<'info, FarmsPda>,

    #[account(mut)]
    pub user_account: Signer<'info>,

    #[account(mut)]
    pub spl_ata_source: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub spl_ata_destination: Box<Account<'info, token::TokenAccount>>,
    pub spl_mint: Box<Account<'info, token::Mint>>,

    #[account(mut)]
    pub field_ata_source: Box<Account<'info, token::TokenAccount>>,
    /// CHECK: We're about to create this
    #[account(mut)]
    pub field_ata_destination: UncheckedAccount<'info>,
    pub field_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: Trifle will check this
    #[account(mut)]
    pub field_metadata: UncheckedAccount<'info>,
    /// CHECK: Trifle will check this
    pub field_master_edition: UncheckedAccount<'info>,

    /// CHECK: Trifle will check this
    #[account(mut)]
    pub escrow_account: UncheckedAccount<'info>,
    /// CHECK: Trifle will check this
    #[account(mut)]
    pub escrow_constraint_model: UncheckedAccount<'info>,
    /// CHECK: Trifle will check this
    #[account(mut)]
    pub trifle_account: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
    /// CHECK: Trifle will check this
    pub trifle_program: UncheckedAccount<'info>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub instructions: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct BuySeed<'info> {
    #[account(seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Account<'info, FarmsPda>,

    #[account(mut)]
    pub user_account: Signer<'info>,

    #[account(mut)]
    pub spl_ata_source: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub spl_ata_destination: Box<Account<'info, token::TokenAccount>>,
    pub spl_mint: Box<Account<'info, token::Mint>>,

    #[account(mut)]
    pub seed_ata_source: Box<Account<'info, token::TokenAccount>>,
    /// CHECK: We're about to create this
    #[account(mut)]
    pub seed_ata_destination: UncheckedAccount<'info>,
    pub seed_mint: Box<Account<'info, token::Mint>>,
    /// CHECK: 
    pub seed_metadata: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
}

#[derive(Accounts)]
pub struct SellCrop<'info> {
    #[account(seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Account<'info, FarmsPda>,

    #[account(mut)]
    pub user_account: Signer<'info>,

    #[account(mut)]
    pub spl_ata_source: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub spl_ata_destination: Box<Account<'info, token::TokenAccount>>,
    pub spl_mint: Box<Account<'info, token::Mint>>,

    #[account(mut)]
    pub crop_ata_source: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub crop_ata_destination: Box<Account<'info, token::TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub crop_metadata: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
}

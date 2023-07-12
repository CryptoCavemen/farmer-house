use {
    crate::{errors::*, program::FarmerHouse as program_farmer_house, FarmsPda},
    anchor_lang::{
        prelude::*,
        solana_program::{
            program::{invoke, invoke_signed},
            sysvar::instructions::ID as INSTRUCTIONS_ID,
        },
    },
    anchor_spl::{associated_token, token},
    mpl_trifle::{
        instruction as trifle_instruction,
        // state::{trifle::Trifle, SolanaAccount, TRIFLE_SEED},
        ID as TRIFLE_PROGRAM_ID,
    },
};

pub fn init_farms_pda(
    ctx: Context<InitFarmsPda>,
    tomato_seed_collection_address: Pubkey,
    tomato_sappling_collection_address: Pubkey,
    ripe_tomato_collection_address: Pubkey,
    field_collection_address: Pubkey,
) -> Result<()> {
    let farms_pda_info = &mut ctx.accounts.farms_pda;

    farms_pda_info.bump = *ctx.bumps.get("farms_pda").unwrap();
    farms_pda_info.authority_address = ctx.accounts.pda_authority.key();
    farms_pda_info.spl_mint_address = ctx.accounts.spl_mint.key();
    farms_pda_info.tomato_seed_collection_address = tomato_seed_collection_address;
    farms_pda_info.tomato_sappling_collection_address = tomato_sappling_collection_address;
    farms_pda_info.ripe_tomato_collection_address = ripe_tomato_collection_address;
    farms_pda_info.field_collection_address = field_collection_address;

    associated_token::create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.pda_authority.to_account_info(),
            associated_token: ctx.accounts.pda_associated_token_account.to_account_info(),
            authority: ctx.accounts.farms_pda.to_account_info(),
            mint: ctx.accounts.spl_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    ))?;
    Ok(())
}

pub fn init_constraint_model(
    ctx: Context<InitConstraintModel>,
    name: String,
    schema_uri: String,
) -> Result<()> {
    let farms_pda_info = &ctx.accounts.farms_pda;

    require!(
        ctx.accounts.pda_authority.is_signer,
        FarmerHouseError::MissingRequiredSignature
    );

    require_eq!(
        ctx.accounts.pda_authority.key(),
        farms_pda_info.authority_address,
        FarmerHouseError::AuthorityMismatch
    );

    require_eq!(
        ctx.accounts.instructions.key(),
        INSTRUCTIONS_ID,
        FarmerHouseError::ProgramMismatch
    );

    require_eq!(
        ctx.accounts.tomato_seed_collection_mint.key(),
        farms_pda_info.tomato_seed_collection_address,
        FarmerHouseError::CollectionMismatch
    );

    msg!("Creating constraint model account");

    invoke(
        &trifle_instruction::create_escrow_constraint_model_account(
            &TRIFLE_PROGRAM_ID,
            &ctx.accounts.escrow_constraint_model.key(),
            &ctx.accounts.pda_authority.key(),
            &ctx.accounts.farms_pda.key(),
            name,
            Some(schema_uri),
        ),
        &[
            ctx.accounts.escrow_constraint_model.to_account_info(),
            ctx.accounts.pda_authority.to_account_info(),
            ctx.accounts.farms_pda.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.instructions.to_account_info(),
        ],
    )?;

    let array = vec!["a1", "a2", "a3", "b1", "b2", "b3"];

    array.iter().for_each(|&x| {
        match invoke_signed(
            &trifle_instruction::add_collection_constraint_to_escrow_constraint_model(
                &TRIFLE_PROGRAM_ID,
                &ctx.accounts.escrow_constraint_model.key(),
                &ctx.accounts.pda_authority.key(),
                &ctx.accounts.farms_pda.key(),
                &ctx.accounts.tomato_seed_collection_mint.key(),
                &ctx.accounts.tomato_seed_collection_metadata.key(),
                x.to_string(),
                1,
                1,
            ),
            &[
                ctx.accounts.escrow_constraint_model.to_account_info(),
                ctx.accounts.pda_authority.to_account_info(),
                ctx.accounts.farms_pda.to_account_info(),
                ctx.accounts.tomato_seed_collection_mint.to_account_info(),
                ctx.accounts
                    .tomato_seed_collection_metadata
                    .to_account_info(),
                ctx.accounts.trifle_program.to_account_info(),
                ctx.accounts.instructions.to_account_info(),
            ],
            &[&[
                b"farmer-house-farms",
                ctx.accounts.farmer_house_program.key().as_ref(),
                &[ctx.accounts.farms_pda.bump],
            ]],
        ) {
            Ok(_) => (),
            Err(e) => msg!("Error: {:?}", e),
        }
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitFarmsPda<'info> {
    #[account(mut)]
    pub pda_authority: Signer<'info>,
    // space:
    // 8 discriminator + 1 bump + 32 authority_address + 32 spl_mint_address + 32 tomato_seed_collection_address
    // + 32 young_tomato_seed_collection_address + 32 ripe_tomato_collection_address + 32 field_collection
    #[account(
        init,
        payer = pda_authority,
        space = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 32,
        seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()],
        bump
    )]
    pub farms_pda: Account<'info, FarmsPda>,
    /// CHECK associated_token_program will check this
    #[account(mut)]
    pub pda_associated_token_account: UncheckedAccount<'info>,
    pub spl_mint: Account<'info, token::Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
}

#[derive(Accounts)]
pub struct InitConstraintModel<'info> {
    /// CHECK: Trifle will check this
    #[account(mut)]
    pub escrow_constraint_model: UncheckedAccount<'info>,
    #[account(seeds = [b"farmer-house-farms", farmer_house_program.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Account<'info, FarmsPda>,
    #[account(mut)]
    pub pda_authority: Signer<'info>,
    /// CHECK: Trifle will check this
    pub tomato_seed_collection_metadata: UncheckedAccount<'info>,
    pub tomato_seed_collection_mint: Account<'info, token::Mint>,

    /// CHECK: Trifle will check this
    pub trifle_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub farmer_house_program: Program<'info, program_farmer_house>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub instructions: UncheckedAccount<'info>,
}

use {
    crate::FarmsPda,
    anchor_lang::{prelude::*, solana_program::program::invoke_signed, system_program},
    anchor_spl::{associated_token, token},
    mpl_token_metadata::{
        instruction::{create_master_edition_v3, create_metadata_accounts_v3},
        state::{Collection, Creator},
        ID as TOKEN_METADATA_ID,
    },
};

pub fn mint(
    ctx: Context<Mint>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
) -> Result<()> {
    msg!("Creating mint account...");
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        Rent::get()?.minimum_balance(token::Mint::LEN),
        token::Mint::LEN as u64,
        &ctx.accounts.token_program.key(),
    )?;
    msg!("Mint account created successfully.");

    msg!("Initializing mint account...");
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        &ctx.accounts.farms_pda.key(),
        Some(&ctx.accounts.farms_pda.key()),
    )?;
    msg!("Mint account initialized successfully.");

    msg!("Creating token account...");
    associated_token::create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.payer.to_account_info(),
            associated_token: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    ))?;
    msg!("Token account created successfully.");

    msg!("Minting token to token account...");
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.farms_pda.to_account_info(),
            },
            &[&[
                b"farmer-house-farms",
                ctx.accounts.pda_authority.key().as_ref(),
                &[ctx.accounts.farms_pda.bump],
            ]],
        ),
        1,
    )?;
    msg!("Token minted");

    // Declare creators vec for metadata
    let creators: Vec<Creator> = vec![Creator {
        address: ctx.accounts.farms_pda.key(),
        verified: true,
        share: 100,
    }];

    // Declare collection for metadata
    let collection = Collection {
        verified: (false),
        key: (ctx.accounts.collection_mint.key()),
    };

    msg!("Creating metadata account...");
    invoke_signed(
        &create_metadata_accounts_v3(
            TOKEN_METADATA_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.farms_pda.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.farms_pda.key(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
            Some(creators),
            1,
            true,
            true,
            Some(collection),
            None,
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.farms_pda.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ],
        &[&[
            b"farmer-house-farms",
            ctx.accounts.pda_authority.key().as_ref(),
            &[ctx.accounts.farms_pda.bump],
        ]],
    )?;
    msg!("Metadata created successfully.");

    msg!("Creating Master Edition...");
    invoke_signed(
        &create_master_edition_v3(
            TOKEN_METADATA_ID,
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.farms_pda.key(),
            ctx.accounts.farms_pda.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            Some(1),
        ),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.farms_pda.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ],
        &[&[
            b"farmer-house-farms",
            ctx.accounts.pda_authority.key().as_ref(),
            &[ctx.accounts.farms_pda.bump],
        ]],
    )?;
    msg!("Master Edition created successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct Mint<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub pda_authority: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"farmer-house-farms", pda_authority.key().as_ref()], bump = farms_pda.bump)]
    pub farms_pda: Account<'info, FarmsPda>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: We're about to create this
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    pub collection_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

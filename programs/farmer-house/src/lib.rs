use anchor_lang::prelude::*;

pub mod errors;
pub mod farm;
pub mod init;
pub mod mint;
pub mod shop;

use farm::*;
use init::*;
use shop::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod farmer_house {
    use super::*;

    pub fn initialize_farms_pda(
        ctx: Context<InitFarmsPda>,
        tomato_seed_collection_address: Pubkey,
        tomato_sappling_collection_address: Pubkey,
        ripe_tomato_collection_address: Pubkey,
        field_collection_address: Pubkey,
    ) -> Result<()> {
        init_farms_pda(
            ctx,
            tomato_seed_collection_address,
            tomato_sappling_collection_address,
            ripe_tomato_collection_address,
            field_collection_address,
        )
    }

    pub fn initialize_constraint_model(
        ctx: Context<InitConstraintModel>,
        name: String,
        schema_uri: String,
    ) -> Result<()> {
        init_constraint_model(ctx, name, schema_uri)
    }

    pub fn plant_seed(ctx: Context<PlantSeed>, crop_slot: String) -> Result<()> {
        farm::plant_seed(ctx, crop_slot)
    }

    pub fn buy_field(ctx: Context<BuyField>) -> Result<()> {
        shop::buy_field(ctx)
    }

    pub fn buy_seed(ctx: Context<BuySeed>) -> Result<()> {
        shop::buy_seed(ctx)
    }

    pub fn water(ctx: Context<Water>, crop_slot: String) -> Result<()> {
        farm::water(ctx, crop_slot)
    }

    pub fn harvest_crop(ctx: Context<HarvestCrop>, crop_slot: String) -> Result<()> {
        farm::harvest_crop(ctx, crop_slot)
    }

    pub fn sell_crop(ctx: Context<SellCrop>) -> Result<()> {
        shop::sell_crop(ctx)
    }
}

#[account]
pub struct FarmsPda {
    bump: u8,
    authority_address: Pubkey,
    spl_mint_address: Pubkey,
    tomato_seed_collection_address: Pubkey,
    tomato_sappling_collection_address: Pubkey,
    ripe_tomato_collection_address: Pubkey,
    field_collection_address: Pubkey,
}

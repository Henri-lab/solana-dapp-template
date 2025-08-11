use anchor_lang::prelude::*;

pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("FqkHhvwGbuFBXmUBXpyDQGgGkU34fnhJxzttH1As4Nw9");

#[program]
pub mod anchor_20250808 {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        instructions::initialize_game(ctx)
    }

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        instructions::initialize_player(ctx)
    }

    pub fn update_score(ctx: Context<UpdateScore>, new_score: u64) -> Result<()> {
        instructions::update_score(ctx, new_score)
    }

    pub fn deactivate_game(ctx: Context<DeactivateGame>) -> Result<()> {
        instructions::deactivate_game(ctx)
    }
}

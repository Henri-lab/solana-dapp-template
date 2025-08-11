use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;

    game_state.authority = ctx.accounts.authority.key();
    game_state.players = 0;
    game_state.total_score = 0;
    game_state.is_active = true;
    game_state.created_at = clock.unix_timestamp;

    msg!("Game initialized by: {:?}", ctx.accounts.authority.key());
    Ok(())
}

pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let clock = Clock::get()?;

    player.authority = ctx.accounts.authority.key();
    player.score = 0;
    player.games_played = 0;
    player.joined_at = clock.unix_timestamp;

    msg!("Player initialized: {:?}", ctx.accounts.authority.key());
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        payer = authority,
        space = GameState::LEN,
        seeds = [b"game", authority.key().as_ref()],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        payer = authority,
        space = Player::LEN,
        seeds = [b"player", authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
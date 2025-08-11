use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

pub fn update_score(ctx: Context<UpdateScore>, new_score: u64) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game_state = &mut ctx.accounts.game_state;

    require!(game_state.is_active, ErrorCode::InvalidOperation);
    require!(player.authority == ctx.accounts.authority.key(), ErrorCode::Unauthorized);

    player.score += new_score;
    player.games_played += 1;
    game_state.total_score += new_score;

    msg!("Score updated for player: {:?}, new score: {}", player.authority, player.score);
    Ok(())
}

pub fn deactivate_game(ctx: Context<DeactivateGame>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;

    require!(game_state.authority == ctx.accounts.authority.key(), ErrorCode::Unauthorized);
    
    game_state.is_active = false;
    
    msg!("Game deactivated by: {:?}", ctx.accounts.authority.key());
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(
        mut,
        seeds = [b"player", authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,
    
    #[account(
        mut,
        seeds = [b"game", game_state.authority.as_ref()],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeactivateGame<'info> {
    #[account(
        mut,
        seeds = [b"game", authority.key().as_ref()],
        bump,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub game_state: Account<'info, GameState>,
    
    pub authority: Signer<'info>,
}
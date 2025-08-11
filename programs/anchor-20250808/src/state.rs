use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub authority: Pubkey,
    pub players: u64,
    pub total_score: u64,
    pub is_active: bool,
    pub created_at: i64,
}

impl GameState {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 + // players
        8 + // total_score
        1 + // is_active
        8; // created_at
}

#[account]
pub struct Player {
    pub authority: Pubkey,
    pub score: u64,
    pub games_played: u64,
    pub joined_at: i64,
}

impl Player {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 + // score
        8 + // games_played
        8; // joined_at
}
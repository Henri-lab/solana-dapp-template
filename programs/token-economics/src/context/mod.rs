use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{TokenEconomics, StakingPool, UserStake};
use crate::errors::EconomicsError;

#[derive(Accounts)]
pub struct InitializeEconomics<'info> {
    #[account(
        init,
        payer = authority,
        space = TokenEconomics::LEN,
        seeds = [b"economics"],
        bump
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Token used for staking
    pub stake_mint: Account<'info, Mint>,
    
    /// Token used for rewards
    pub reward_mint: Account<'info, Mint>,
    
    /// Treasury vault for collecting fees
    #[account(
        init,
        payer = authority,
        associated_token::mint = reward_mint,
        associated_token::authority = economics
    )]
    pub treasury_vault: Account<'info, TokenAccount>,
    
    /// Vault for storing staked tokens
    #[account(
        init,
        payer = authority,
        associated_token::mint = stake_mint,
        associated_token::authority = economics
    )]
    pub stake_vault: Account<'info, TokenAccount>,
    
    /// Vault for reward token distribution
    #[account(
        init,
        payer = authority,
        associated_token::mint = reward_mint,
        associated_token::authority = economics
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(pool_id: u8)]
pub struct CreateStakingPool<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump,
        has_one = authority
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(
        init,
        payer = authority,
        space = StakingPool::LEN,
        seeds = [b"pool", pool_id.to_le_bytes().as_ref()],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Fixed: Proper bump handling for init_if_needed
#[derive(Accounts)]
#[instruction(pool_id: u8)]
pub struct StakeTokens<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(
        mut,
        seeds = [b"pool", pool_id.to_le_bytes().as_ref()],
        bump = staking_pool.bump,
        constraint = staking_pool.is_active @ EconomicsError::PoolNotActive
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        init,
        payer = user,
        space = UserStake::LEN,
        seeds = [b"user_stake", user.key().as_ref(), pool_id.to_le_bytes().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// User's token account for the stake token
    #[account(
        mut,
        associated_token::mint = economics.stake_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// Vault where staked tokens are stored
    #[account(
        mut,
        associated_token::mint = economics.stake_mint,
        associated_token::authority = economics
    )]
    pub stake_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(
        mut,
        seeds = [b"pool", user_stake.pool_id.to_le_bytes().as_ref()],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [b"user_stake", user.key().as_ref(), user_stake.pool_id.to_le_bytes().as_ref()],
        bump = user_stake.bump,
        has_one = user
    )]
    pub user_stake: Account<'info, UserStake>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// User's reward token account
    #[account(
        mut,
        associated_token::mint = economics.reward_mint,
        associated_token::authority = user
    )]
    pub user_reward_account: Account<'info, TokenAccount>,
    
    /// Treasury vault for governance fees
    #[account(
        mut,
        associated_token::mint = economics.reward_mint,
        associated_token::authority = economics
    )]
    pub treasury_vault: Account<'info, TokenAccount>,
    
    /// Reward vault for token distribution
    #[account(
        mut,
        associated_token::mint = economics.reward_mint,
        associated_token::authority = economics
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(
        mut,
        seeds = [b"pool", user_stake.pool_id.to_le_bytes().as_ref()],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [b"user_stake", user.key().as_ref(), user_stake.pool_id.to_le_bytes().as_ref()],
        bump = user_stake.bump,
        has_one = user
    )]
    pub user_stake: Account<'info, UserStake>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// User's token account for receiving unstaked tokens
    #[account(
        mut,
        associated_token::mint = economics.stake_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// Vault where staked tokens are stored
    #[account(
        mut,
        associated_token::mint = economics.stake_mint,
        associated_token::authority = economics
    )]
    pub stake_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EmergencyUnstake<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(
        mut,
        seeds = [b"user_stake", user.key().as_ref(), user_stake.pool_id.to_le_bytes().as_ref()],
        bump = user_stake.bump,
        has_one = user
    )]
    pub user_stake: Account<'info, UserStake>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        associated_token::mint = economics.stake_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = economics.stake_mint,
        associated_token::authority = economics
    )]
    pub stake_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AdminControl<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump,
        has_one = authority
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct FundRewards<'info> {
    #[account(
        mut,
        seeds = [b"economics"],
        bump = economics.bump,
        has_one = authority
    )]
    pub economics: Account<'info, TokenEconomics>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        associated_token::mint = economics.reward_mint,
        associated_token::authority = authority
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = economics.reward_mint,
        associated_token::authority = economics
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}
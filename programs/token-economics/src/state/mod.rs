use anchor_lang::prelude::*;

/// Main economics configuration and global state
#[account]
pub struct TokenEconomics {
    /// Program authority (admin)
    pub authority: Pubkey,
    
    /// Token mint for staking
    pub stake_mint: Pubkey,
    
    /// Token mint for rewards
    pub reward_mint: Pubkey,
    
    /// Treasury vault for governance fees
    pub treasury_vault: Pubkey,
    
    /// Vault for storing staked tokens
    pub stake_vault: Pubkey,
    
    /// Vault for reward distribution
    pub reward_vault: Pubkey,
    
    // Economic parameters
    /// Base reward rate per second per staked token
    pub reward_rate_per_second: u64,
    
    /// Governance fee in basis points (100 = 1%)
    pub governance_fee_bps: u16,
    
    /// Minimum amount required to stake
    pub min_stake_amount: u64,
    
    /// Maximum amount allowed to stake per user
    pub max_stake_amount: u64,
    
    // Global state
    /// Total amount of tokens staked across all pools
    pub total_staked: u64,
    
    /// Total rewards distributed to users
    pub total_rewards_distributed: u64,
    
    /// Last time rewards were updated
    pub last_reward_update_time: i64,
    
    /// Global accumulated reward per token
    pub accumulated_reward_per_token: u64,
    
    /// Number of active stakers
    pub active_stakers: u64,
    
    // System flags
    /// Whether the system is paused
    pub is_paused: bool,
    
    /// Whether emergency mode is enabled
    pub emergency_mode: bool,
    
    /// When the economics system was created
    pub created_at: i64,
    
    /// PDA bump seed
    pub bump: u8,
}

impl TokenEconomics {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // stake_mint
        32 + // reward_mint
        32 + // treasury_vault
        32 + // stake_vault
        32 + // reward_vault
        8 +  // reward_rate_per_second
        2 +  // governance_fee_bps
        8 +  // min_stake_amount
        8 +  // max_stake_amount
        8 +  // total_staked
        8 +  // total_rewards_distributed
        8 +  // last_reward_update_time
        8 +  // accumulated_reward_per_token
        8 +  // active_stakers
        1 +  // is_paused
        1 +  // emergency_mode
        8 +  // created_at
        1;   // bump
}

/// Individual staking pool with custom parameters
#[account]
pub struct StakingPool {
    /// Unique pool identifier
    pub pool_id: u8,
    
    /// Reference to the main economics account
    pub economics: Pubkey,
    
    /// Reward multiplier for this pool (100 = 1x, 200 = 2x)
    pub reward_multiplier: u16,
    
    /// Minimum time tokens must be staked (seconds)
    pub min_stake_period: i64,
    
    /// Maximum total capacity for this pool
    pub max_capacity: u64,
    
    /// Current total staked in this pool
    pub total_staked: u64,
    
    /// Number of active stakers in this pool
    pub active_stakers: u32,
    
    /// Pool-specific accumulated reward per token
    pub accumulated_reward_per_token: u64,
    
    /// When this pool was created
    pub created_at: i64,
    
    /// Whether this pool is active
    pub is_active: bool,
    
    /// PDA bump seed
    pub bump: u8,
}

impl StakingPool {
    pub const LEN: usize = 8 + // discriminator
        1 +  // pool_id
        32 + // economics
        2 +  // reward_multiplier
        8 +  // min_stake_period
        8 +  // max_capacity
        8 +  // total_staked
        4 +  // active_stakers
        8 +  // accumulated_reward_per_token
        8 +  // created_at
        1 +  // is_active
        1;   // bump
}

/// Individual user's stake information for a specific pool
#[account]
pub struct UserStake {
    /// User's wallet address
    pub user: Pubkey,
    
    /// Pool this stake belongs to
    pub pool_id: u8,
    
    /// Amount of tokens currently staked
    pub total_staked: u64,
    
    /// Pending rewards not yet claimed
    pub pending_rewards: u64,
    
    /// Reward debt (for reward calculation)
    pub reward_debt: u64,
    
    /// Total rewards claimed by this user
    pub total_rewards_claimed: u64,
    
    /// When user first staked in this pool
    pub first_stake_time: i64,
    
    /// When user last staked tokens
    pub last_stake_time: i64,
    
    /// When user last claimed rewards
    pub last_claim_time: i64,
    
    /// PDA bump seed
    pub bump: u8,
}

impl UserStake {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 +  // pool_id
        8 +  // total_staked
        8 +  // pending_rewards
        8 +  // reward_debt
        8 +  // total_rewards_claimed
        8 +  // first_stake_time
        8 +  // last_stake_time
        8 +  // last_claim_time
        1;   // bump
}
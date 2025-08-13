use anchor_lang::prelude::*;

pub mod state;
pub mod context;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod utils;

use context::*;

declare_id!("E8zKxLztD2HbMcGVdnZVzfXBgKPnfxYUJFKtPkH9oNn8");
/// Token Economics Program
/// 
/// A comprehensive staking and reward system showcasing:
/// - SPL Token integration and management
/// - Complex mathematical reward calculations
/// - Multi-tier staking pools with different reward rates
/// - Governance token distribution mechanics
/// - Emergency pause and upgrade mechanisms
/// - Gas-optimized batch operations
#[program]
pub mod token_economics {
    use super::*;

    /// Initialize the token economics system with configurable parameters
    pub fn initialize_economics(
        ctx: Context<InitializeEconomics>,
        reward_rate_per_second: u64,
        governance_fee_bps: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
    ) -> Result<()> {
        instructions::initialize_economics(ctx, reward_rate_per_second, governance_fee_bps, min_stake_amount, max_stake_amount)
    }

    /// Create a new staking pool with custom parameters
    pub fn create_staking_pool(
        ctx: Context<CreateStakingPool>,
        pool_id: u8,
        reward_multiplier: u16,
        min_stake_period: i64,
        max_capacity: u64,
    ) -> Result<()> {
        instructions::create_staking_pool(ctx, pool_id, reward_multiplier, min_stake_period, max_capacity)
    }

    /// Stake tokens into a specific pool
    pub fn stake_tokens(
        ctx: Context<StakeTokens>,
        pool_id: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::stake_tokens(ctx, pool_id, amount)
    }

    /// Claim accumulated rewards from staking
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }

    /// Unstake tokens from the pool
    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        instructions::unstake_tokens(ctx, amount)
    }

    /// Emergency unstake (admin only)
    pub fn emergency_unstake(ctx: Context<EmergencyUnstake>) -> Result<()> {
        instructions::emergency_unstake(ctx)
    }

    /// Admin functions for system management

    /// Pause/unpause the system
    pub fn set_pause_state(ctx: Context<AdminControl>, is_paused: bool) -> Result<()> {
        instructions::set_pause_state(ctx, is_paused)
    }

    /// Enable/disable emergency mode
    pub fn set_emergency_mode(ctx: Context<AdminControl>, emergency_mode: bool) -> Result<()> {
        instructions::set_emergency_mode(ctx, emergency_mode)
    }

    /// Update reward rate (admin only)
    pub fn update_reward_rate(ctx: Context<AdminControl>, new_rate: u64) -> Result<()> {
        instructions::update_reward_rate(ctx, new_rate)
    }

    /// Fund the reward vault with tokens (admin only)
    pub fn fund_rewards(ctx: Context<FundRewards>, amount: u64) -> Result<()> {
        instructions::fund_rewards(ctx, amount)
    }
}
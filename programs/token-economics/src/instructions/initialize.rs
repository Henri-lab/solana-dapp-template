use anchor_lang::prelude::*;
use crate::context::{InitializeEconomics, CreateStakingPool};
use crate::events::{PoolCreatedEvent};
use crate::errors::EconomicsError;

pub fn initialize_economics(
    ctx: Context<InitializeEconomics>,
    reward_rate_per_second: u64,
    governance_fee_bps: u16,
    min_stake_amount: u64,
    max_stake_amount: u64,
) -> Result<()> {
    require!(governance_fee_bps <= 10000, EconomicsError::InvalidFeeRate);
    require!(min_stake_amount > 0, EconomicsError::InvalidStakeAmount);
    require!(max_stake_amount >= min_stake_amount, EconomicsError::InvalidStakeAmount);

    let economics = &mut ctx.accounts.economics;
    let clock = Clock::get()?;

    economics.authority = ctx.accounts.authority.key();
    economics.stake_mint = ctx.accounts.stake_mint.key();
    economics.reward_mint = ctx.accounts.reward_mint.key();
    economics.treasury_vault = ctx.accounts.treasury_vault.key();
    economics.stake_vault = ctx.accounts.stake_vault.key();
    economics.reward_vault = ctx.accounts.reward_vault.key();
    
    // Reward configuration
    economics.reward_rate_per_second = reward_rate_per_second;
    economics.governance_fee_bps = governance_fee_bps;
    economics.min_stake_amount = min_stake_amount;
    economics.max_stake_amount = max_stake_amount;
    
    // Global state
    economics.total_staked = 0;
    economics.total_rewards_distributed = 0;
    economics.last_reward_update_time = clock.unix_timestamp;
    economics.accumulated_reward_per_token = 0;
    economics.active_stakers = 0;
    
    // System flags
    economics.is_paused = false;
    economics.emergency_mode = false;
    economics.created_at = clock.unix_timestamp;
    economics.bump = ctx.bumps.economics;

    msg!("Token Economics initialized with reward rate: {} tokens/second", reward_rate_per_second);
    
    Ok(())
}

pub fn create_staking_pool(
    ctx: Context<CreateStakingPool>,
    pool_id: u8,
    reward_multiplier: u16,
    min_stake_period: i64,
    max_capacity: u64,
) -> Result<()> {
    require!(!ctx.accounts.economics.is_paused, EconomicsError::SystemPaused);
    require!(reward_multiplier > 0 && reward_multiplier <= 1000, EconomicsError::InvalidMultiplier);
    require!(min_stake_period >= 0, EconomicsError::InvalidStakePeriod);

    let pool = &mut ctx.accounts.staking_pool;
    let clock = Clock::get()?;

    pool.pool_id = pool_id;
    pool.economics = ctx.accounts.economics.key();
    pool.reward_multiplier = reward_multiplier;
    pool.min_stake_period = min_stake_period;
    pool.max_capacity = max_capacity;
    pool.total_staked = 0;
    pool.active_stakers = 0;
    pool.created_at = clock.unix_timestamp;
    pool.is_active = true;
    pool.bump = ctx.bumps.staking_pool;

    emit!(PoolCreatedEvent {
        pool_id,
        reward_multiplier,
        min_stake_period,
        max_capacity,
    });

    Ok(())
}
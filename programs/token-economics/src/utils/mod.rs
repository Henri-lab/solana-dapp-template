use anchor_lang::prelude::*;
use crate::state::{TokenEconomics, StakingPool, UserStake};
use crate::errors::EconomicsError;

/// Updates the accumulated reward per token based on time elapsed
pub fn update_reward_accumulation(
    economics: &mut Account<TokenEconomics>,
    pool: &mut Account<StakingPool>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let time_delta = current_time - economics.last_reward_update_time;
    
    if time_delta > 0 && pool.total_staked > 0 {
        // Calculate rewards for this time period
        let base_rewards = (economics.reward_rate_per_second as u128)
            .checked_mul(time_delta as u128)
            .ok_or(EconomicsError::MathOverflow)?;
            
        // Apply pool multiplier
        let pool_rewards = base_rewards
            .checked_mul(pool.reward_multiplier as u128)
            .ok_or(EconomicsError::MathOverflow)?
            .checked_div(100)
            .ok_or(EconomicsError::MathOverflow)?;
            
        // Update accumulated reward per token (scaled by 1e12 for precision)
        let reward_per_token_delta = pool_rewards
            .checked_mul(1_000_000_000_000)
            .ok_or(EconomicsError::MathOverflow)?
            .checked_div(pool.total_staked as u128)
            .ok_or(EconomicsError::MathOverflow)?;
            
        pool.accumulated_reward_per_token = pool.accumulated_reward_per_token
            .checked_add(reward_per_token_delta as u64)
            .ok_or(EconomicsError::MathOverflow)?;
    }
    
    economics.last_reward_update_time = current_time;
    Ok(())
}

/// Calculates pending rewards for a user based on their stake and reward debt
pub fn calculate_pending_rewards(
    user_stake: &Account<UserStake>,
    pool: &Account<StakingPool>,
) -> Result<u64> {
    if user_stake.total_staked == 0 {
        return Ok(0);
    }
    
    let earned = (user_stake.total_staked as u128)
        .checked_mul(pool.accumulated_reward_per_token as u128)
        .ok_or(EconomicsError::MathOverflow)?
        .checked_div(1_000_000_000_000)
        .ok_or(EconomicsError::MathOverflow)? as u64;
        
    let pending = earned.checked_sub(user_stake.reward_debt).unwrap_or(0);
    Ok(pending)
}

/// Calculates the reward debt for a given stake amount
pub fn calculate_reward_debt(
    stake_amount: u64,
    pool: &Account<StakingPool>,
) -> Result<u64> {
    let debt = (stake_amount as u128)
        .checked_mul(pool.accumulated_reward_per_token as u128)
        .ok_or(EconomicsError::MathOverflow)?
        .checked_div(1_000_000_000_000)
        .ok_or(EconomicsError::MathOverflow)? as u64;
    Ok(debt)
}
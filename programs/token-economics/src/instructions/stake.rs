use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::context::{StakeTokens, UnstakeTokens, EmergencyUnstake};
use crate::events::{StakeEvent, UnstakeEvent, EmergencyUnstakeEvent};
use crate::errors::EconomicsError;
use crate::utils::{update_reward_accumulation, calculate_pending_rewards, calculate_reward_debt};

pub fn stake_tokens(
    ctx: Context<StakeTokens>,
    pool_id: u8,
    amount: u64,
) -> Result<()> {
    require!(!ctx.accounts.economics.is_paused, EconomicsError::SystemPaused);
    require!(amount >= ctx.accounts.economics.min_stake_amount, EconomicsError::BelowMinimumStake);
    
    let economics = &mut ctx.accounts.economics;
    let pool = &mut ctx.accounts.staking_pool;
    let user_stake = &mut ctx.accounts.user_stake;
    let clock = Clock::get()?;

    // Validate pool capacity
    require!(pool.total_staked + amount <= pool.max_capacity, EconomicsError::PoolCapacityExceeded);
    
    // Validate user maximum stake
    require!(
        user_stake.total_staked + amount <= economics.max_stake_amount,
        EconomicsError::ExceedsMaximumStake
    );

    // Update global reward accumulation
    update_reward_accumulation(economics, pool)?;

    // Calculate and store pending rewards before updating stake
    let pending_rewards = calculate_pending_rewards(user_stake, pool)?;
    user_stake.pending_rewards = user_stake.pending_rewards
        .checked_add(pending_rewards)
        .ok_or(EconomicsError::MathOverflow)?;

    // Initialize user stake if first time
    if user_stake.total_staked == 0 {
        user_stake.user = ctx.accounts.user.key();
        user_stake.pool_id = pool_id;
        user_stake.first_stake_time = clock.unix_timestamp;
        user_stake.bump = ctx.bumps.user_stake;
        pool.active_stakers = pool.active_stakers.checked_add(1).ok_or(EconomicsError::MathOverflow)?;
        economics.active_stakers = economics.active_stakers.checked_add(1).ok_or(EconomicsError::MathOverflow)?;
    }

    // Update stake amounts
    user_stake.total_staked = user_stake.total_staked
        .checked_add(amount)
        .ok_or(EconomicsError::MathOverflow)?;
    user_stake.last_stake_time = clock.unix_timestamp;
    
    // Calculate new reward debt
    user_stake.reward_debt = calculate_reward_debt(user_stake.total_staked, pool)?;

    // Update pool and global totals
    pool.total_staked = pool.total_staked
        .checked_add(amount)
        .ok_or(EconomicsError::MathOverflow)?;
    economics.total_staked = economics.total_staked
        .checked_add(amount)
        .ok_or(EconomicsError::MathOverflow)?;

    // Transfer tokens from user to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.stake_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    emit!(StakeEvent {
        user: ctx.accounts.user.key(),
        pool_id,
        amount,
        total_user_stake: user_stake.total_staked,
        total_pool_stake: pool.total_staked,
    });

    Ok(())
}

pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
    require!(!ctx.accounts.economics.emergency_mode, EconomicsError::EmergencyMode);

    let economics = &mut ctx.accounts.economics;
    let pool = &mut ctx.accounts.staking_pool;
    let user_stake = &mut ctx.accounts.user_stake;
    let clock = Clock::get()?;

    require!(user_stake.total_staked > 0, EconomicsError::NoStakeToUnstake);

    // Check minimum staking period
    let stake_duration = clock.unix_timestamp - user_stake.last_stake_time;
    require!(stake_duration >= pool.min_stake_period, EconomicsError::MinimumStakePeriodNotMet);

    // Determine actual unstake amount
    let unstake_amount = if amount == 0 { 
        user_stake.total_staked 
    } else { 
        amount 
    };
    
    require!(unstake_amount <= user_stake.total_staked, EconomicsError::InsufficientStake);

    // Update reward accumulation and claim pending rewards
    update_reward_accumulation(economics, pool)?;
    let pending_rewards = calculate_pending_rewards(user_stake, pool)?;
    user_stake.pending_rewards = user_stake.pending_rewards
        .checked_add(pending_rewards)
        .ok_or(EconomicsError::MathOverflow)?;

    // Update stake amounts
    user_stake.total_staked = user_stake.total_staked
        .checked_sub(unstake_amount)
        .ok_or(EconomicsError::MathOverflow)?;

    // If fully unstaking, handle active staker count
    if user_stake.total_staked == 0 {
        pool.active_stakers = pool.active_stakers.checked_sub(1).ok_or(EconomicsError::MathOverflow)?;
        economics.active_stakers = economics.active_stakers.checked_sub(1).ok_or(EconomicsError::MathOverflow)?;
    }

    // Calculate new reward debt
    user_stake.reward_debt = calculate_reward_debt(user_stake.total_staked, pool)?;

    // Update pool and global totals
    pool.total_staked = pool.total_staked
        .checked_sub(unstake_amount)
        .ok_or(EconomicsError::MathOverflow)?;
    economics.total_staked = economics.total_staked
        .checked_sub(unstake_amount)
        .ok_or(EconomicsError::MathOverflow)?;

    // Transfer tokens back to user
    let seeds = &[b"economics".as_ref(), &[economics.bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.stake_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.economics.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, unstake_amount)?;

    emit!(UnstakeEvent {
        user: ctx.accounts.user.key(),
        pool_id: user_stake.pool_id,
        amount: unstake_amount,
        remaining_stake: user_stake.total_staked,
    });

    Ok(())
}

pub fn emergency_unstake(ctx: Context<EmergencyUnstake>) -> Result<()> {
    let economics = &mut ctx.accounts.economics;
    let user_stake = &mut ctx.accounts.user_stake;

    require!(economics.emergency_mode, EconomicsError::NotInEmergencyMode);
    require!(user_stake.total_staked > 0, EconomicsError::NoStakeToUnstake);

    let unstake_amount = user_stake.total_staked;
    user_stake.total_staked = 0;
    user_stake.pending_rewards = 0;
    user_stake.reward_debt = 0;

    // Update global totals
    economics.total_staked = economics.total_staked
        .checked_sub(unstake_amount)
        .ok_or(EconomicsError::MathOverflow)?;

    // Transfer tokens back to user
    let seeds = &[b"economics".as_ref(), &[economics.bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.stake_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.economics.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, unstake_amount)?;

    emit!(EmergencyUnstakeEvent {
        user: ctx.accounts.user.key(),
        amount: unstake_amount,
    });

    Ok(())
}
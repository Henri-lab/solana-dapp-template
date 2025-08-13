use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::context::ClaimRewards;
use crate::events::RewardClaimEvent;
use crate::errors::EconomicsError;
use crate::utils::{update_reward_accumulation, calculate_pending_rewards, calculate_reward_debt};

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    require!(!ctx.accounts.economics.is_paused, EconomicsError::SystemPaused);
    require!(!ctx.accounts.economics.emergency_mode, EconomicsError::EmergencyMode);

    let economics = &mut ctx.accounts.economics;
    let pool = &mut ctx.accounts.staking_pool;
    let user_stake = &mut ctx.accounts.user_stake;

    // Update reward accumulation
    update_reward_accumulation(economics, pool)?;

    // Calculate total claimable rewards
    let pending_rewards = calculate_pending_rewards(user_stake, pool)?;
    let total_rewards = user_stake.pending_rewards
        .checked_add(pending_rewards)
        .ok_or(EconomicsError::MathOverflow)?;

    require!(total_rewards > 0, EconomicsError::NoRewardsToClaim);

    // Calculate governance fee
    let governance_fee = total_rewards
        .checked_mul(economics.governance_fee_bps as u64)
        .ok_or(EconomicsError::MathOverflow)?
        .checked_div(10000)
        .ok_or(EconomicsError::MathOverflow)?;

    let user_reward = total_rewards
        .checked_sub(governance_fee)
        .ok_or(EconomicsError::MathOverflow)?;

    // Reset user's reward state
    user_stake.pending_rewards = 0;
    user_stake.reward_debt = calculate_reward_debt(user_stake.total_staked, pool)?;
    user_stake.total_rewards_claimed = user_stake.total_rewards_claimed
        .checked_add(user_reward)
        .ok_or(EconomicsError::MathOverflow)?;
    user_stake.last_claim_time = Clock::get()?.unix_timestamp;

    // Update global statistics
    economics.total_rewards_distributed = economics.total_rewards_distributed
        .checked_add(total_rewards)
        .ok_or(EconomicsError::MathOverflow)?;

    // Create signer seeds for PDA
    let seeds = &[b"economics".as_ref(), &[economics.bump]];
    let signer_seeds = &[&seeds[..]];

    // Transfer governance fee to treasury
    if governance_fee > 0 {
        let treasury_cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.treasury_vault.to_account_info(),
            authority: ctx.accounts.economics.to_account_info(),
        };
        let treasury_cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            treasury_cpi_accounts,
            signer_seeds,
        );
        token::transfer(treasury_cpi_ctx, governance_fee)?;
    }

    // Transfer user rewards
    let user_cpi_accounts = Transfer {
        from: ctx.accounts.reward_vault.to_account_info(),
        to: ctx.accounts.user_reward_account.to_account_info(),
        authority: ctx.accounts.economics.to_account_info(),
    };
    let user_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        user_cpi_accounts,
        signer_seeds,
    );
    token::transfer(user_cpi_ctx, user_reward)?;

    emit!(RewardClaimEvent {
        user: ctx.accounts.user.key(),
        pool_id: user_stake.pool_id,
        gross_reward: total_rewards,
        governance_fee,
        net_reward: user_reward,
    });

    Ok(())
}
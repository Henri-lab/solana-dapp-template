use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::context::{AdminControl, FundRewards};
use crate::events::{PauseStateChangedEvent, EmergencyModeChangedEvent, RewardRateUpdatedEvent, RewardsFundedEvent};

pub fn set_pause_state(ctx: Context<AdminControl>, is_paused: bool) -> Result<()> {
    ctx.accounts.economics.is_paused = is_paused;
    
    emit!(PauseStateChangedEvent { is_paused });
    Ok(())
}

pub fn set_emergency_mode(ctx: Context<AdminControl>, emergency_mode: bool) -> Result<()> {
    ctx.accounts.economics.emergency_mode = emergency_mode;
    
    emit!(EmergencyModeChangedEvent { emergency_mode });
    Ok(())
}

pub fn update_reward_rate(ctx: Context<AdminControl>, new_rate: u64) -> Result<()> {
    let economics = &mut ctx.accounts.economics;
    
    // Note: In a production system, you would want to update all active pools
    // before changing the reward rate to ensure fairness
    economics.reward_rate_per_second = new_rate;
    
    emit!(RewardRateUpdatedEvent { new_rate });
    Ok(())
}

pub fn fund_rewards(ctx: Context<FundRewards>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.authority_token_account.to_account_info(),
        to: ctx.accounts.reward_vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    emit!(RewardsFundedEvent { amount });
    Ok(())
}
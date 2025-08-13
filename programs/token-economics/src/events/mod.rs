use anchor_lang::prelude::*;

#[event]
pub struct PoolCreatedEvent {
    pub pool_id: u8,
    pub reward_multiplier: u16,
    pub min_stake_period: i64,
    pub max_capacity: u64,
}

#[event]
pub struct StakeEvent {
    pub user: Pubkey,
    pub pool_id: u8,
    pub amount: u64,
    pub total_user_stake: u64,
    pub total_pool_stake: u64,
}

#[event]
pub struct UnstakeEvent {
    pub user: Pubkey,
    pub pool_id: u8,
    pub amount: u64,
    pub remaining_stake: u64,
}

#[event]
pub struct RewardClaimEvent {
    pub user: Pubkey,
    pub pool_id: u8,
    pub gross_reward: u64,
    pub governance_fee: u64,
    pub net_reward: u64,
}

#[event]
pub struct EmergencyUnstakeEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct PauseStateChangedEvent {
    pub is_paused: bool,
}

#[event]
pub struct EmergencyModeChangedEvent {
    pub emergency_mode: bool,
}

#[event]
pub struct RewardRateUpdatedEvent {
    pub new_rate: u64,
}

#[event]
pub struct RewardsFundedEvent {
    pub amount: u64,
}
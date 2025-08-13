use anchor_lang::prelude::*;

#[error_code]
pub enum EconomicsError {
    #[msg("Invalid fee rate - must be <= 10000 basis points")]
    InvalidFeeRate,
    
    #[msg("Invalid stake amount configuration")]
    InvalidStakeAmount,
    
    #[msg("Invalid reward multiplier")]
    InvalidMultiplier,
    
    #[msg("Invalid stake period")]
    InvalidStakePeriod,
    
    #[msg("System is currently paused")]
    SystemPaused,
    
    #[msg("System is in emergency mode")]
    EmergencyMode,
    
    #[msg("Not in emergency mode")]
    NotInEmergencyMode,
    
    #[msg("Pool capacity exceeded")]
    PoolCapacityExceeded,
    
    #[msg("Pool is not active")]
    PoolNotActive,
    
    #[msg("Amount below minimum stake requirement")]
    BelowMinimumStake,
    
    #[msg("Amount exceeds maximum stake limit")]
    ExceedsMaximumStake,
    
    #[msg("Insufficient staked amount")]
    InsufficientStake,
    
    #[msg("No stake to unstake")]
    NoStakeToUnstake,
    
    #[msg("Minimum stake period not met")]
    MinimumStakePeriodNotMet,
    
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    
    #[msg("Mathematical overflow occurred")]
    MathOverflow,
    
    #[msg("Unauthorized access")]
    Unauthorized,
}
use anchor_lang::prelude::*; // Import Anchor framework essentials

// Global staking configuration - stored as a Program Derived Account (PDA)
// This struct defines the rules and parameters for the entire staking program
#[account] // Marks this as an Anchor account that can be stored on-chain
#[derive(InitSpace)] // Automatically calculates space needed for account storage
pub struct StakeConfig {
    pub points_per_stake: u8, // How many reward points earned per staking period
    pub max_stake: u8, // Maximum number of NFTs a single user can stake simultaneously  
    pub freeze_period: u32, // Minimum time (in seconds) NFTs must stay staked before unstaking
    pub rewards_bump: u8, // PDA bump seed for the rewards token mint account
    pub bump: u8, // PDA bump seed for this config account itself
}


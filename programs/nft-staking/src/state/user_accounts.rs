use anchor_lang::prelude::*; // Import Anchor framework essentials

// User's staking account - tracks their individual staking statistics
// Each user gets one account created when they first interact with the program  
#[account] // Marks this as an Anchor account that can be stored on-chain
#[derive(InitSpace)] // Automatically calculates space needed for account storage
pub struct UserAccount {
    pub points: u32, // Total reward points accumulated from staking (claimable as tokens)
    pub amount_staked: u8, // Current number of NFTs this user has staked
    pub bump: u8, // PDA bump seed for this user account
}
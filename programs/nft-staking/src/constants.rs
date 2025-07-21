use anchor_lang::prelude::*; // Import Anchor framework essentials

// Declare a program constant that can be used in instruction contexts
#[constant]
pub const SEED: &str = "anchor"; // Seed string for generating Program Derived Addresses (PDAs)
                                // Currently unused but available for future PDA generation needs

# NFT Staking Program

A Solana-based NFT staking program that allows users to stake their NFTs and earn reward tokens over time.

## What is NFT Staking?

NFT Staking is a mechanism where NFT holders can "lock up" their NFTs in a smart contract for a specified period to earn rewards. Think of it like a savings account for your NFTs - you deposit them, they stay locked for a minimum time, and you earn interest in the form of tokens.

### Key Concepts

- **Staking**: Locking your NFT in the program to start earning rewards
- **Freeze Period**: Minimum time your NFT must stay locked before you can unstake
- **Reward Points**: Points accumulated while your NFT is staked
- **Claiming**: Converting your accumulated points into actual reward tokens
- **Unstaking**: Removing your NFT from the program after the freeze period

## Program Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    NFT STAKING PROGRAM                      │
└─────────────────────────────────────────────────────────────┘

┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   GLOBAL CONFIG  │    │   USER ACCOUNT   │    │  STAKE ACCOUNT   │
│                  │    │                  │    │                  │
│ • Points/Stake   │    │ • Total Points   │    │ • NFT Owner      │
│ • Max Stake      │    │ • Amount Staked  │    │ • NFT Mint       │
│ • Freeze Period  │    │ • PDA Bump       │    │ • Staked Time    │
│ • Rewards Bump   │    │                  │    │ • PDA Bump       │
│ • PDA Bump       │    │                  │    │                  │
└──────────────────┘    └──────────────────┘    └──────────────────┘
        │                        │                        │
        │                        │                        │
        ▼                        ▼                        ▼
   (One per program)      (One per user)         (One per staked NFT)
```

## User Flow Diagram

```
USER JOURNEY

1. INITIALIZE USER ACCOUNT
   User creates their staking account
   ┌─────────────────┐
   │   User Wallet   │
   └─────────────────┘
            │
            ▼
   ┌─────────────────┐
   │ User Account    │
   │ Points: 0       │
   │ Staked: 0       │
   └─────────────────┘

2. STAKE NFT
   User locks their NFT to start earning
   ┌─────────────────┐      ┌─────────────────┐
   │   NFT in Wallet │ ──►  │ NFT Frozen      │
   │   (Transferable)│      │ (Locked)        │
   └─────────────────┘      └─────────────────┘
            │                        │
            ▼                        ▼
   ┌─────────────────┐      ┌─────────────────┐
   │ User Account    │      │ Stake Record    │
   │ Points: 0       │      │ Owner: User     │
   │ Staked: 1       │      │ Mint: NFT       │
   └─────────────────┘      │ Time: Now       │
                            └─────────────────┘

3. EARN REWARDS (Automatic over time)
   Points accumulate based on staking duration

4. UNSTAKE NFT (After freeze period)
   User gets NFT back + points added to account
   ┌─────────────────┐      ┌─────────────────┐
   │ NFT Frozen      │ ──►  │   NFT in Wallet │
   │ (Locked)        │      │   (Transferable)│
   └─────────────────┘      └─────────────────┘
            │                        │
            ▼                        ▼
   ┌─────────────────┐      ┌─────────────────┐
   │ User Account    │      │ Stake Record    │
   │ Points: +10     │      │ [DELETED]       │
   │ Staked: 0       │      │                 │
   └─────────────────┘      └─────────────────┘

5. CLAIM REWARDS
   Convert points to actual tokens
   ┌─────────────────┐      ┌─────────────────┐
   │ User Account    │ ──►  │ Reward Tokens   │
   │ Points: 0       │      │ in Wallet       │
   │ Staked: 0       │      │                 │
   └─────────────────┘      └─────────────────┘
```

## Technical Flow

### Staking Process
```
STAKE INSTRUCTION FLOW

1. Verify NFT Collection
   ├─ Check metadata belongs to expected collection
   └─ Verify collection is verified by creator

2. Verify User Limits
   └─ Check user hasn't exceeded max_stake limit

3. Create Stake Record
   ├─ Store: owner, mint, timestamp, bump
   └─ PDA: ["stake", nft_mint, config]

4. Delegate NFT Control
   ├─ Approve stake account as delegate
   └─ Freeze NFT (prevents transfers)

5. Update User Stats
   └─ Increment amount_staked counter
```

### Unstaking Process  
```
UNSTAKE INSTRUCTION FLOW

1. Verify Freeze Period
   └─ Check: current_time - staked_at >= freeze_period

2. Verify User Has Stakes
   └─ Check: amount_staked > 0

3. Award Points
   └─ Add points_per_stake to user's points

4. Transfer NFT Back
   ├─ Unfreeze and transfer NFT to user
   └─ Config PDA signs as authority

5. Update User Stats
   ├─ Decrement amount_staked
   └─ Close stake account (return rent)
```

## Code Structure

```
nft-staking/
├── programs/nft-staking/src/
│   ├── lib.rs                 # Main program entry point
│   ├── constants.rs           # Program constants
│   ├── error.rs              # Custom error definitions
│   ├── state/                # Account data structures
│   │   ├── mod.rs            #    State module exports
│   │   ├── stake_config.rs   #    Global program settings
│   │   ├── user_accounts.rs  #    User staking statistics  
│   │   └── stake_account.rs  #    Individual NFT records
│   └── instructions/         # Instruction handlers
│       ├── mod.rs           #    Instructions module exports
│       ├── initialize_config.rs    # Admin setup
│       ├── initialize_user_accounts.rs # User onboarding
│       ├── stake.rs         #    NFT staking logic
│       ├── unstake.rs       #    NFT unstaking logic
│       └── claim.rs         #    Reward claiming logic
├── tests/
│   └── nft-staking.ts       # Program tests
└── README.md                # This file
```

## Account Relationships

```
PROGRAM DERIVED ADDRESSES (PDAs)

Global Config (One per program)
┌─────────────────────────────────────┐
│ PDA: ["config"]                     │
│ Authority for: Reward Token Mint    │
└─────────────────────────────────────┘

Reward Token Mint (One per program)  
┌─────────────────────────────────────┐
│ PDA: ["rewards", config.pubkey]     │
│ Mint Authority: Config PDA          │
└─────────────────────────────────────┘

User Account (One per user)
┌─────────────────────────────────────┐
│ PDA: ["user", user.pubkey]          │
│ Stores: points, amount_staked       │
└─────────────────────────────────────┘

Stake Account (One per staked NFT)
┌─────────────────────────────────────┐
│ PDA: ["stake", nft_mint, config]    │
│ Stores: owner, mint, staked_at      │
│ Delegate Authority for: NFT         │
└─────────────────────────────────────┘
```

## Instructions Overview

| Instruction | Who Can Call | Purpose | Accounts Created |
|------------|--------------|---------|------------------|
| `initialize_config` | Admin | Set up global parameters | Config, Rewards Mint |
| `initialize_user` | Anyone | Create user staking account | User Account |
| `stake` | NFT Owner | Lock NFT and start earning | Stake Account |
| `unstake` | NFT Owner | Unlock NFT and claim points | (Closes Stake Account) |
| `claim` | Anyone | Convert points to tokens | (None) |

## Security Features

- **Collection Verification**: Only NFTs from verified collections can be staked
- **Freeze Period**: Prevents immediate unstaking (reduces manipulation)  
- **Overflow Protection**: Safe math prevents arithmetic errors
- **PDA Authority**: Program controls locked NFTs through PDAs
- **Rent Recovery**: Closed accounts return rent to users

## Getting Started

### Prerequisites
- Solana CLI tools
- Anchor framework
- Node.js and npm/yarn

### Build and Test
```bash
# Build the program
anchor build

# Run tests
anchor test

# Deploy to devnet (optional)
anchor deploy --provider.cluster devnet
```

### Configuration Parameters
When initializing, set these parameters:
- `points_per_stake`: How many points earned per staking period (e.g., 10)
- `max_stake`: Maximum NFTs per user (e.g., 5) 
- `freeze_period`: Minimum staking time in seconds (e.g., 86400 = 24 hours)

## Usage Examples

### Initialize Program (Admin)
```typescript
await program.methods.initializeConfig(
  10,    // points_per_stake
  5,     // max_stake  
  86400  // freeze_period (24 hours)
).accounts({
  admin: adminWallet.publicKey
}).rpc();
```

### Create User Account
```typescript
await program.methods.initializeUser().accounts({
  user: userWallet.publicKey
}).rpc();
```

### Stake NFT
```typescript
await program.methods.stake().accounts({
  user: userWallet.publicKey,
  mint: nftMint.publicKey,
  collectionMint: collectionMint.publicKey,
  // ... other required accounts
}).rpc();
```

### Unstake NFT (after freeze period)
```typescript
await program.methods.unstake().accounts({
  user: userWallet.publicKey,
  nftMint: nftMint.publicKey,
  // ... other required accounts  
}).rpc();
```

### Claim Rewards
```typescript
await program.methods.claim().accounts({
  user: userWallet.publicKey,
  // ... other required accounts
}).rpc();
```

## Error Codes

| Error | Description | When It Occurs |
|-------|-------------|----------------|
| `TimeNotElapsed` | Freeze period not over | Unstaking too early |
| `MaxStake` | Staking limit reached | User tries to stake too many NFTs |
| `Underflow` | Arithmetic underflow | Math operation would go negative |
| `Overflow` | Arithmetic overflow | Math operation exceeds limits |


---

This project is licensed under the MIT License.

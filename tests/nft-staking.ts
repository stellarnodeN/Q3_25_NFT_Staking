// Import Anchor framework for Solana program testing
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor"; // Type for strongly-typed program interface
import { NftStaking } from "../target/types/nft_staking"; // Generated TypeScript types for the program

// Test suite for the NFT staking program
describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env()); // Use environment provider (typically localhost)

  // Get a typed program interface for making instruction calls
  const program = anchor.workspace.nftStaking as Program<NftStaking>;

  // Test case for initializing the global staking configuration
  it("Is initialized!", async () => {
    // Add your test here.
    const provider = anchor.getProvider(); // Get the current provider to access wallet
    
    // Call the initialize_config instruction with test parameters
    const tx = await program.methods.initializeConfig(10, 5, 86400).accounts({
      admin: provider.publicKey, // Use provider's public key as admin
    }).rpc(); // Send transaction and wait for confirmation
    
    console.log("Your transaction signature", tx); // Log the transaction signature for verification
  });
});

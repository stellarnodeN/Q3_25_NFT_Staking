import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.nftStaking as Program<NftStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const provider = anchor.getProvider();
    const tx = await program.methods.initializeConfig(10, 5, 86400).accounts({
      admin: provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});

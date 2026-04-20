import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Accountinfo } from "../target/types/accountinfo";

describe("accountinfo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.accountinfo as Program<Accountinfo>;
   // generate a signer to call our function
  let myKeypair = anchor.web3.Keypair.generate();
  let myKeypair2 = anchor.web3.Keypair.generate();

  it("Is called by the owner", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      signerAccount: myKeypair.publicKey,
  }).rpc();
    console.log("Transaction Hash: ", tx);
  });
});

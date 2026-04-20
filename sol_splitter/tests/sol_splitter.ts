import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolSplitter } from "../target/types/sol_splitter";

describe("sol_splitter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solSplitter as Program<SolSplitter>;

  async function printAccountBalance(account){
    const balance = await anchor.getProvider().connection.getBalance(account);
    console.log(`${account} has ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  }

  it("Transmit SOL", async () => {
    //generate a new wallet
    const recipient = anchor.web3.Keypair.generate();
    const recipientb = anchor.web3.Keypair.generate();

    await printAccountBalance(recipient.publicKey);
    await printAccountBalance(recipientb.publicKey);

    let amount1 = new anchor.BN(1  * anchor.web3.LAMPORTS_PER_SOL); // 1 SOL
    let amount2 = new anchor.BN(3  * anchor.web3.LAMPORTS_PER_SOL); // 1 SOL


    // Add your test here.
    const tx = await program.methods.sendSol(amount1, amount2).accounts({recipient: recipient.publicKey, recipientb: recipientb.publicKey}).rpc();

    await printAccountBalance(recipient.publicKey);
    await printAccountBalance(recipientb.publicKey);
    console.log("Your transaction signature", tx);
  });
});

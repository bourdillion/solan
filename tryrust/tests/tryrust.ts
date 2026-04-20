import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tryrust } from "../target/types/tryrust";
import { BN } from "bn.js";

describe("tryrust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tryrust as Program<Tryrust>;

  it("Is matchAge", async () => {
    // Add your test here.
    const tx = await program.methods.matchAge(new anchor.BN(9)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is loopOver", async() => {
     const tx = await program.methods.loopOver().rpc();
     console.log("Your transaction signature", tx);
  });

  it("Is arrayOver", async() => {
     const tx = await program.methods.arrayOver().rpc();
     console.log("Your transaction signature", tx);
  });

  it("Is vectorOver", async() => {
     const tx = await program.methods.dynamicArray().rpc();
     console.log("Your transaction signature", tx);
  });

  it("Is mapping", async() => {
     const tx = await program.methods.mapping("name", "Bob").rpc();
     console.log("Your transaction signature", tx);
  });

  it("Is struct", async() => {
     const tx = await program.methods.structs("Ayo", new anchor.BN(26) ).rpc();
     console.log("Your transaction signature", tx);
  });

  it("exercise array", async() => {
     const tx = await program.methods.checkEven().rpc();
     console.log("Your transaction signature", tx);
  });
});

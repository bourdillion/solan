import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "hello").rpc();
    console.log("Your transaction signature", tx);
  });

  it("array test", async () => {
      const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
      console.log("Your transaction signature", tx);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.subtract(new anchor.BN(50), new anchor.BN(40)).rpc();
    console.log("Your transaction signature", tx);
  });


  it("Is cbrt", async () => {
    // Add your test here.
    const tx = await program.methods.cbrt(new anchor.BN(125)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is calculator", async () => {
    // Add your test here.
    const tx = await program.methods.calculator(new anchor.BN(64), new anchor.BN(40)).rpc();
    console.log("Your transaction signature", tx);
  });

});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Purse } from "../target/types/purse";

describe("purse", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Purse as Program<Purse>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

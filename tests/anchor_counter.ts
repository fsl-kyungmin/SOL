import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorCounter } from "../target/types/anchor_counter";

describe("anchor-counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;

  const counter = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counter.publicKey,
      })
      .signers([counter])
      .rpc();

    // biome-ignore lint/complexity/useLiteralKeys: <explanation>
    const account = await program.account["counter"].fetch(counter.publicKey);
    expect(account.count.toNumber() === 0);
  });

  it("Incremented the count", async () => {
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // biome-ignore lint/complexity/useLiteralKeys: <explanation>
    const account = await program.account["counter"].fetch(counter.publicKey);
    expect(account.count.toNumber() === 1);
  });

  it("Decremented the count", async () => {
    const tx = await program.methods
      .decrement()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    // biome-ignore lint/complexity/useLiteralKeys: <explanation>
    const account = await program.account["counter"].fetch(counter.publicKey);
    expect(account.count.toNumber() === 0);
  });
});

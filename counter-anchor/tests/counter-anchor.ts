import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterAnchor } from "../target/types/counter_anchor";
import { assert } from "chai";

describe("counter-anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterAnchor as Program<CounterAnchor>;

  // Generate a new account for the counter
  const counterAccount = anchor.web3.Keypair.generate();

  it("Initialize counter", async () => {
    await program.methods
      .initialize()
      .accounts({
        counterAccount: counterAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([counterAccount])
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.equal(account.counter.toString(), "0");
    console.log("Initialized counter to:", account.counter.toString());
  });

  it("Increment counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counterAccount: counterAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.equal(account.counter.toString(), "1");
    console.log("Incremented counter to:", account.counter.toString());
  });

  it("Decrement counter", async () => {
    await program.methods
      .decrement()
      .accounts({
        counterAccount: counterAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.equal(account.counter.toString(), "0");
    console.log("Decremented counter to:", account.counter.toString());
  });

  it("Set counter", async () => {
    const newValue = new anchor.BN(42);
    await program.methods
      .set(newValue)
      .accounts({
        counterAccount: counterAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.equal(account.counter.toString(), newValue.toString());
    console.log("Set counter to:", account.counter.toString());
  });
});

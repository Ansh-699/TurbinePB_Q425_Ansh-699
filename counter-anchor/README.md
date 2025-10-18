# Counter Anchor Program

A minimal Solana program built with Anchor that manages a persistent counter account with initialize, increment, decrement, and set operations.

Note: Tests currently initialize successfully but other instructions may be missing in the generated IDL. See the Troubleshooting section for the current state and how to proceed.

## Prerequisites

- Solana CLI
- Anchor CLI 0.31.1
- Node.js 18+ and Yarn
- Rust toolchain

Verify versions:

```sh
solana --version
anchor --version
node --version
yarn --version
rustc --version
```

## Project layout

- programs/counter-anchor/src/lib.rs: On-chain program logic
- Anchor.toml: Workspace configuration and test script
- migrations/: Scripts for deployment flows (if any)
- tests/counter-anchor.ts: Mocha tests using Anchor TS client
- target/idl/: Generated IDL
- target/types/: TypeScript type helper generated from IDL

## Install dependencies

```sh
cd counter-anchor
yarn install
```

## Build the program

```sh
anchor build
```

This compiles the program and generates the IDL in `target/idl/counter_anchor.json`.

## Run tests

Localnet is used via the Anchor provider settings in `Anchor.toml`.

```sh
anchor test
```

If successful, you should see the initialize test passing. If increment/decrement/set tests fail due to missing methods in the client, see Troubleshooting.

## Client usage example (TypeScript)

```ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterAnchor } from "../target/types/counter_anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.CounterAnchor as Program<CounterAnchor>;

const counter = anchor.web3.Keypair.generate();

await program.methods
  .initialize()
  .accounts({
    counterAccount: counter.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([counter])
  .rpc();
```

## Troubleshooting

- program.methods.increment is not a function: The generated IDL may only contain the `initialize` instruction. Rebuild the program and regenerate the IDL:
  - `anchor clean && anchor build`
  - `anchor idl build > target/idl/counter_anchor.json`
  - Ensure additional instructions exist in `programs/counter-anchor/src/lib.rs` and use distinct account contexts as needed.
- Local validator issues: Restart the validator or run tests again.
- Version mismatches: Ensure `@coral-xyz/anchor` and Anchor CLI versions align (0.31.1).

## License

ISC (see package.json)

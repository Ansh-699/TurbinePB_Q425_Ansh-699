# RNG Game Workspace

This folder contains three small Rust CLI projects you can build and run independently:

- hello_world: Minimal program that prints a greeting.
- guessing_game: A number guessing game using random numbers.
- counter: An interactive counter REPL supporting simple commands.

## Prerequisites

- Rust toolchain (rustup + cargo)
- Linux shell (these examples use bash/zsh compatible commands)

Install Rust if needed:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then ensure your PATH is updated (restart the shell or run the snippet shown by rustup).

## Project structure

- hello_world/
  - src/main.rs: Prints two lines.
- guessing_game/
  - src/main.rs: Generates a random number and prompts for guesses.
  - Cargo.toml: Uses the `rand` crate.
- counter/
  - src/main.rs: Simple REPL to increment/decrement a counter.

## Build and run

From this folder, change into each crate and run cargo commands.

### hello_world

```sh
cd "RNG Game/hello_world"
cargo run
```

Expected output:

```
-----Hello, world!-----
i love turbin
```

### guessing_game

```sh
cd "RNG Game/guessing_game"
cargo run
```

- The program picks a random number between 0 and 100.
- Enter guesses; it will report Too high!/Too low!/Correct with attempt count.

If you see dependency download/compile on first run, that's normal.

### counter (CLI REPL)

```sh
cd "RNG Game/counter"
cargo run
```

Commands:

- + or inc: Increment the counter by 1
- - or dec: Decrement the counter by 1
- add N: Add N to the counter
- sub N: Subtract N from the counter
- reset: Reset to 0
- show: Print current counter
- quit: Exit the program (CTRL+C if not available)

Example session:

```
---------------------COUNTER-----------------------
Commands: + | - | add N | sub N | reset | show | quit
COUNT IS  = 0 
> +
count = 1
> add 10
count = 11
> sub 3
count = 8
> show
count = 8
```

## Troubleshooting

- cargo not found: Ensure Rust was installed and PATH updated.
- Build errors: Run `cargo clean` and re-try, or update toolchain with `rustup update`.
- Random number support: guessing_game uses `rand = "0.9.2"` as specified in Cargo.toml.

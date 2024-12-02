# powdrVM Usage Template

This is a foundational template for generating zero-knowledge proofs with powdrVM.
You write the code to be proven as a guest program for the zkVM host.
This template includes a structure for host/guest interaction, ZKP setup,
and artifact generation.

Guest programs are written in Rust.
When creating your guest program, you can write Rust code in the usual way,
including using std and importing packages others have written.
We provide some additional powdrVM specific functionalities via system calls,
such as IO operations for host <-> guest communication and precompiles to
accelerate complex programs via optimized circuits.

## Dependencies

- Rust/cargo toolchain nightly-2024-09-21. Example installation:
```console
rustup toolchain install nightly-2024-09-21-x86_64-unknown-linux-gnu
```

## Usage

This will run the host and generate ZK proofs.
It must be run from the root of `any-zkvm` due to the relative path to
the guest program.

```bash
cargo run -r -p powdrVM
```

## AVX / Neon

You can enable AVX or Neon if your hardware supports it:

```bash
RUSTFLAGS='-C target-cpu=native' cargo run -r -p powdrVM
```

## Structure

- `src/main.rs`: the host code. This is where you create a powdr `Session`,
prepare data to be shared with the guest, and run the prover.
- `guest`: this is the guest crate. It contains the code that will be
run inside the powdrVM.
- `powdr-target`: this is where all generated artifacts reside.
This includes the compiled guest code to powdr-asm, the compiled PIL constraints,
setup artifacts such as proving and verifying keys, and the final ZK proofs.

## Workflow

Let's look at `src/main.rs` line by line:

Here we read the data we want to share with the guest:

```rust
let input = Processor::get_host_inputs();
```

Create a new powdr session where we'll be running crate `guest` in powdrVM
and all artifacts will be stored in `powdr-target`:

```rust
let mut session = Session::builder()
    .guest_path("./guest")
    .out_path("powdr-target")
    .build()
```

Write `input` to channel 0.
Note that any `serde` type can be used to share data between host and guest.

The guest will read this data from the channels:

```rust
    .write(0, &input);
```

Run the session without generating a proof. Useful for testing the guest code:

```rust
session.run();
```

Generate the ZK proof:

```rust
session.prove();
```

Before generating a proof, powdrVM has to create the proving and verifying keys (setup)
for the given guest program.
When run for the first time, this can take a while.
Subsequent runs will be faster as the setup only changes if the guest changes.

powdrVM also needs to compute the witnesses for the given execution trace,
needed by the ZK prover.
Currently this is done by an automated constraint solver,
which can be slow for complex programs.
We are working on a more efficient way to generate witnesses.

You can run the host with INFO logs to have a deeper look at what's happening:

```bash
RUST_LOG=info cargo run -r
```

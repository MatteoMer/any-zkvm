use lib::{Processor, ZkvmProcessor};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

pub const FIBONACCI_ELF: &[u8] = include_elf!("sp1-program");

fn main() {
    sp1_sdk::utils::setup_logger();
    let client = ProverClient::new();

    // Setup the inputs.
    let inputs = Processor::get_host_inputs();
    let mut stdin = SP1Stdin::new();
    stdin.write(&inputs);

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    println!("[sp1] Generating proof!");
    // Generate the proof
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("[sp1] Successfully verified proof!");
}

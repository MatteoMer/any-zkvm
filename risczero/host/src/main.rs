use lib::{Processor, RiscZeroZkvmProcessor, ZkvmProcessor};
use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input = Processor::get_host_inputs();
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    println!("[risczero] Generating proof!");
    let prove_info = prover.prove(env, METHOD_ELF).unwrap();

    let receipt = prove_info.receipt;

    receipt.verify(METHOD_ID).unwrap();
    println!("[risczero] Successfully verified proof!");

    let outputs = Processor::process_internal_outputs(&receipt);
    Processor::process_outputs(outputs);
}

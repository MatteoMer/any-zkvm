use lib::{Processor, ZkvmProcessor};
use risc0_zkvm::guest::env;

fn main() {
    let input = Processor::get_guest_inputs().expect("[risc0] couldn't load inputs");
    let output = Processor::prove(input).expect("[risc0] error during proving");
    env::commit(&output);
}

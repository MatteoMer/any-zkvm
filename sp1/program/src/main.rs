#![no_main]
sp1_zkvm::entrypoint!(main);

use lib::{Processor, ZkvmProcessor};

pub fn main() {
    let inputs = Processor::get_guest_inputs().expect("[sp1] error while getting guest inputs");

    let outputs: <Processor as ZkvmProcessor>::Output =
        Processor::prove(inputs).expect("[sp1] error while proving");

    sp1_zkvm::io::commit::<<Processor as ZkvmProcessor>::Output>(&outputs);
}

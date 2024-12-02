use lib::{Processor, ZkvmProcessor};

use powdr_riscv_runtime;

fn main() {
    let input = Processor::get_guest_inputs().expect("[powdr] couldn't load inputs");
    let output = Processor::prove(input).expect("[powdr] error during proving");

    powdr_riscv_runtime::io::write(3, &output);

    powdr_riscv_runtime::commit::commit(output.0);
    powdr_riscv_runtime::commit::commit(output.1);
    powdr_riscv_runtime::commit::commit(output.2);
}

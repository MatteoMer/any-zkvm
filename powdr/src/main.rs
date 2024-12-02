use lib::{Processor, ZkvmProcessor};

use powdr::Session;

fn main() {
    env_logger::init();

    let input = Processor::get_host_inputs();

    // Create a new powdr session to make proofs for the `guest` crate.
    // Store all temporary and final artifacts in `powdr-target`.
    let mut session = Session::builder()
        .guest_path("./powdr/guest")
        .out_path("powdr-target")
        .build()
        // Write `input` to channel 0.
        // Any serde-serializable type can be written to a channel.
        .write(0, &input);

    // Fast dry run to test execution.
    // Useful for testing before running the full proof.
    session.run();

    // Compute the proof.
    session.prove();

    // TODO implement when powdr Session exposes the output via Session
    //let outputs = Processor::process_internal_outputs();
    //Processor::process_outputs(outputs);
}

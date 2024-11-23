pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}

#[derive(Debug)]
pub enum ZkvmProcessError {
    NotImplemented,
}

/*
  You can change the definitions here to fit the needs of your program
*/
pub trait ZkvmProcessor {
    type Output;
    type Input

    fn get_guest_inputs() -> Result<u32, ZkvmProcessError>;
    fn get_host_inputs() -> u32;
    fn prove(input: u32) -> Result<Self::Output, ZkvmProcessError>;
    fn process_outputs(output: Self::Output);
}

#[cfg(feature = "risczero")]
pub trait RiscZeroZkvmProcessor {
    fn process_internal_outputs(
        receipt: &risc0_zkvm::Receipt,
    ) -> <Processor as ZkvmProcessor>::Output;
}

#[cfg(feature = "sdk-sp1")]
pub trait Sp1ZkvmProcessor {
    fn process_internal_outputs(
        receipt: &mut sp1_sdk::SP1PublicValues,
    ) -> <Processor as ZkvmProcessor>::Output;
}

#[derive(Debug)]
pub struct Processor;

impl ZkvmProcessor for Processor {
    type Output = (u32, u32, u32);
    type Input = u32;
    fn get_guest_inputs() -> Result<u32, ZkvmProcessError> {
        if cfg!(feature = "sp1") {
            #[cfg(feature = "sp1")]
            {
                Ok(sp1_zkvm::io::read::<Self::Input>())
            }
            #[cfg(not(feature = "sp1"))]
            unreachable!()
        } else if cfg!(feature = "risczero") {
            #[cfg(feature = "risczero")]
            {
                Ok(risc0_zkvm::guest::env::read::<Self::Input>())
            }
            #[cfg(not(feature = "risczero"))]
            unreachable!()
        } else {
            Err(ZkvmProcessError::NotImplemented)
        }
    }

    fn get_host_inputs() -> Self::Input {
        20
    }

    fn prove(input: u32) -> Result<<Processor as ZkvmProcessor>::Output, ZkvmProcessError> {
        let (a, b) = fibonacci(input);
        Ok((a, b, input))
    }

    fn process_outputs(_output: Self::Output) {
        todo!()
    }
}

#[cfg(feature = "risczero")]
impl RiscZeroZkvmProcessor for Processor {
    fn process_internal_outputs(
        receipt: &risc0_zkvm::Receipt,
    ) -> <Processor as ZkvmProcessor>::Output {
        receipt
            .journal
            .decode::<<Processor as ZkvmProcessor>::Output>()
            .expect("[risc0] cannot decode journal")
    }
}

#[cfg(feature = "sdk-sp1")]
impl Sp1ZkvmProcessor for Processor {
    fn process_internal_outputs(
        public_values: &mut sp1_sdk::SP1PublicValues,
    ) -> <Processor as ZkvmProcessor>::Output {
        public_values.read::<<Processor as ZkvmProcessor>::Output>()
    }
}

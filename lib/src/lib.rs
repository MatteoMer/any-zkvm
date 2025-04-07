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
    type Input;

    fn get_guest_inputs() -> Result<Self::Input, ZkvmProcessError>;
    fn get_host_inputs() -> Self::Input;
    fn prove(input: Self::Input) -> Result<Self::Output, ZkvmProcessError>;
    fn process_outputs(output: Self::Output);
}

#[cfg(feature = "sdk-sp1")]
pub trait Sp1ZkvmProcessor {
    fn process_internal_outputs(
        receipt: &mut sp1_sdk::SP1PublicValues,
    ) -> <Processor as ZkvmProcessor>::Output;

    // for SP1 we can use a pre-compile for this function
    fn secp256k1_add(p: *mut [u32; 16], q: *mut [u32; 16]);
}

#[cfg(feature = "risczero")]
pub trait RiscZeroZkvmProcessor {
    fn process_internal_outputs(
        receipt: &risc0_zkvm::Receipt,
    ) -> <Processor as ZkvmProcessor>::Output;

    // While not using it for risc0
    fn secp256k1_add(p: *mut [u32; 16], q: *mut [u32; 16]);
}

#[cfg(feature = "powdr")]
pub trait PowdrZkvmProcessor {
    fn process_internal_outputs(receipt: [u32; 8]) -> <Processor as ZkvmProcessor>::Output;

    // for powdr we can use a pre-compile for this function
    fn secp256k1_add(p: *mut [u32; 16], q: *mut [u32; 16]);
}

#[derive(Debug)]
pub struct Processor;

impl ZkvmProcessor for Processor {
    // TODO: change to your desired input/outputs types
    type Output = (u32, u32, u32);
    type Input = u32;

    //
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
        } else if cfg!(feature = "powdr") {
            #[cfg(all(feature = "powdr", target_os = "zkvm", target_arch = "riscv32"))]
            {
                Ok(powdr_riscv_runtime::io::read::<Self::Input>(0))
            }
            #[cfg(not(all(feature = "powdr", target_os = "zkvm", target_arch = "riscv32")))]
            unreachable!()
        } else {
            Err(ZkvmProcessError::NotImplemented)
        }
    }

    fn get_host_inputs() -> Self::Input {
        3
    }

    fn prove(input: Self::Input) -> Result<<Processor as ZkvmProcessor>::Output, ZkvmProcessError> {
        let (a, b) = fibonacci(input);
        Ok((a, b, input))
    }

    fn process_outputs(output: Self::Output) {
        println!("[any-zkvm] output: {:?}", output);
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

    fn secp256k1_add(_p: *mut [u32; 16], _q: *mut [u32; 16]) {
        todo!()
    }
}

#[cfg(feature = "sdk-sp1")]
impl Sp1ZkvmProcessor for Processor {
    fn process_internal_outputs(
        public_values: &mut sp1_sdk::SP1PublicValues,
    ) -> <Processor as ZkvmProcessor>::Output {
        public_values.read::<<Processor as ZkvmProcessor>::Output>()
    }

    fn secp256k1_add(p: *mut [u32; 16], q: *mut [u32; 16]) {
        sp1_zkvm::syscalls::syscall_secp256k1_add(p, q)
    }
}

#[cfg(feature = "powdr")]
impl PowdrZkvmProcessor for Processor {
    fn process_internal_outputs(_public_values: [u32; 8]) -> <Processor as ZkvmProcessor>::Output {
        Default::default()
    }

    fn secp256k1_add(_p: *mut [u32; 16], _q: *mut [u32; 16]) {
        todo!()
    }
}

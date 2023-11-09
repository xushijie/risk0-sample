// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use hello_methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, serde::{from_slice, to_vec}};

fn main() {


    let a: u64 = 17;
    let b: u64 = 2;

    // First, we construct an executor environment
    //let env = ExecutorEnv::builder().build().unwrap();
    let env = ExecutorEnv::builder().add_input(&to_vec(&a).unwrap())
        .add_input(&to_vec(&b).unwrap())
        .build().unwrap();



    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, MULTIPLY_ELF).unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(MULTIPLY_ID).unwrap();

    let c: u64 = from_slice(&receipt.journal).unwrap();
    println!("Hello, i know the factors of {} and I can prove it", c);
}

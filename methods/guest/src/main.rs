#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    let a: u64 = env::read();
    let b: u64 = env::read();
    if  a == 1 || b == 1 {
        panic! ("trivial factors");
    }
    let product = a.checked_mul(b).expect("Integer overflow");
    env::commit(&product);
}

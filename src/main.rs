#![allow(dead_code)]

use crate::machine::{Machine, TMachine};

pub mod machine;

fn main() {
    // Create a new machine instance
    let mut machine: Machine = machine::Machine::new();

    // Set some registers
    machine.set_register(machine::Register::A, 10);
    machine.set_register(machine::Register::B, 20);

    // Get and print the values of the registers
    println!("Register A: {}", machine.get_register(machine::Register::A));
    println!("Register B: {}", machine.get_register(machine::Register::B));

    // You can add more operations or tests here as needed
}

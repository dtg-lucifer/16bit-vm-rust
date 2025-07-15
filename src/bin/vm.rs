use rustyvm::{Machine, Register};

fn main() -> Result<(), String> {
    let mut vm = Machine::new();

    /*
     * PUSH 10
     * PUSH 8
     * ADDSTACK
     * POPREGISTER A
     */

    // INSTRUCTION 1: PUSH 10
    vm.memory.write(0, 0x1); // Opcode: 0x1 = PUSH
    vm.memory.write(1, 10); // Argument: 10 (value to push)

    // INSTRUCTION 2: PUSH 8
    vm.memory.write(2, 0x1); // Opcode: 0x1 = PUSH
    vm.memory.write(3, 8); // Argument: 8 (value to push)

    // INSTRUCTION 3: ADDSTACK (adds the top two values on the stack)
    vm.memory.write(4, 0x3); // Opcode: 0x3 = ADDSTACK
    vm.memory.write(5, 0); // No argument needed for ADDSTACK

    // INSTRUCTION 4: POPREGISTER A (pop stack into register A)
    vm.memory.write(6, 0x2); // Opcode: 0x2 = POPREGISTER
    vm.memory.write(7, 0); // Argument: 0 = Register A (see Register enum)

    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}

use rustyvm::{Machine, Register};

fn main() -> Result<(), String> {
    let mut vm = Machine::new();

    // Demo set of instructions:
    //
    // PUSH 10
    // PUSH 8
    // ADDSTACK
    // POPREGISTER A
    //
    // 16-bit method example (commented out):
    //
    // Structure each instruction as a 16-bit value with opcode in lower 8 bits and args in upper 8 bits
    //
    // vm.memory.write2(0, 0x1 | (10 << 8)); // PUSH 10 (0x0A01)
    // vm.memory.write2(2, 0x1 | (8 << 8));  // PUSH 8 (0x0801)
    // vm.memory.write2(4, 0x3);             // ADDSTACK (0x0003)
    // vm.memory.write2(6, 0x2 | (0 << 8));  // POPREGISTER A (0x0002)

    // INSTRUCTION 1: PUSH 10
    vm.memory.write(0, 0x1); // Opcode: 0x1 = PUSH
    vm.memory.write(1, 10); // Argument: 10 (value to push)
    // 16-bit equivalent: vm.memory.write2(0, 0x0A01); // 0x01 = PUSH, 0x0A = 10

    // INSTRUCTION 2: PUSH 8
    vm.memory.write(2, 0x1); // Opcode: 0x1 = PUSH
    vm.memory.write(3, 8); // Argument: 8 (value to push)
    // 16-bit equivalent: vm.memory.write2(2, 0x0801); // 0x01 = PUSH, 0x08 = 8

    // INSTRUCTION 3: ADDSTACK (adds the top two values on the stack)
    vm.memory.write(4, 0x3); // Opcode: 0x3 = ADDSTACK
    vm.memory.write(5, 0); // No argument needed for ADDSTACK
    // 16-bit equivalent: vm.memory.write2(4, 0x0003); // 0x03 = ADDSTACK, 0x00 = unused

    // INSTRUCTION 4: POPREGISTER A (pop stack into register A)
    vm.memory.write(6, 0x2); // Opcode: 0x2 = POPREGISTER
    vm.memory.write(7, 0); // Argument: 0 = Register A (see Register enum)
    // 16-bit equivalent: vm.memory.write2(6, 0x0002); // 0x02 = POPREGISTER, 0x00 = Register A

    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {}", vm.get_register(Register::A));

    // Example of a more complex program with 16-bit writes (commented out):
    // Reset VM state
    //
    // let mut vm = Machine::new();
    //
    // Program to calculate (5 + 3) * 2
    //
    // vm.memory.write2(0, 0x0501);  // PUSH 5
    // vm.memory.write2(2, 0x0301);  // PUSH 3
    // vm.memory.write2(4, 0x0003);  // ADDSTACK    -> 8 on stack
    // vm.memory.write2(6, 0x0201);  // PUSH 2
    // vm.memory.write2(8, 0x????);  // MULSTACK    -> 16 on stack (would need to implement)
    // vm.memory.write2(10, 0x0002); // POPREG A    -> A = 16

    // // Execution of complex program would be:
    // // for _ in 0..6 {
    // //    vm.step()?;
    // // }

    Ok(())
}

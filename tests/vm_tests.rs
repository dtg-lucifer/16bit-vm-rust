use rustyvm::{Machine, Op, Register};

#[test]
fn test_push_pop_register() {
    // Create a new VM
    let mut vm = Machine::new();

    // Simple program:
    // PUSH #42
    // POP A
    vm.memory.write(0, Op::Push(0).value()); // PUSH opcode
    vm.memory.write(1, 42); // Value 42
    vm.memory.write(2, Op::PopRegister(Register::A).value()); // POP opcode
    vm.memory.write(3, Register::A as u8); // Register A index

    // Execute the program
    vm.step().expect("Failed to execute PUSH instruction");
    vm.step().expect("Failed to execute POP instruction");

    // Check the result - Register A should contain 42
    assert_eq!(vm.get_register(Register::A), 42);
}

#[test]
fn test_add_stack() {
    // Create a new VM
    let mut vm = Machine::new();

    // Program:
    // PUSH #10
    // PUSH #20
    // ADDSTACK
    // POP A
    vm.memory.write(0, Op::Push(0).value());
    vm.memory.write(1, 10);
    vm.memory.write(2, Op::Push(0).value());
    vm.memory.write(3, 20);
    vm.memory.write(4, Op::AddStack.value());
    vm.memory.write(5, 0); // Not used
    vm.memory.write(6, Op::PopRegister(Register::A).value());
    vm.memory.write(7, Register::A as u8);

    // Execute instructions
    vm.step().expect("Failed to execute PUSH #10");
    vm.step().expect("Failed to execute PUSH #20");
    vm.step().expect("Failed to execute ADDSTACK");
    vm.step().expect("Failed to execute POP A");

    // Register A should contain 30 (10 + 20)
    assert_eq!(vm.get_register(Register::A), 30);
}

#[test]
fn test_multiple_registers() {
    // Create a new VM
    let mut vm = Machine::new();

    // Program:
    // PUSH #10
    // POP A
    // PUSH #20
    // POP B
    // PUSH #30
    // POP C
    let program = [
        Op::Push(0).value(),
        10,
        Op::PopRegister(Register::A).value(),
        Register::A as u8,
        Op::Push(0).value(),
        20,
        Op::PopRegister(Register::B).value(),
        Register::B as u8,
        Op::Push(0).value(),
        30,
        Op::PopRegister(Register::C).value(),
        Register::C as u8,
    ];

    // Load program into memory
    for (i, &byte) in program.iter().enumerate() {
        vm.memory.write(i as u16, byte);
    }

    // Execute the program (6 instructions)
    for _ in 0..6 {
        vm.step().expect("Failed to execute instruction");
    }

    // Check if registers contain expected values
    assert_eq!(vm.get_register(Register::A), 10);
    assert_eq!(vm.get_register(Register::B), 20);
    assert_eq!(vm.get_register(Register::C), 30);
}

#[test]
fn test_program_from_register_asm() {
    // This test simulates running the program in prog/register_asm
    let mut vm = Machine::new();

    // The assembly program:
    // PUSH #10
    // PUSH #20
    // PUSH #30
    // POP B
    // POP C
    // POP A
    // SIG $09
    let program = [
        Op::Push(0).value(),
        10,
        Op::Push(0).value(),
        20,
        Op::Push(0).value(),
        30,
        Op::PopRegister(Register::B).value(),
        Register::B as u8,
        Op::PopRegister(Register::C).value(),
        Register::C as u8,
        Op::PopRegister(Register::A).value(),
        Register::A as u8,
        Op::Signal(0).value(),
        9,
    ];

    // Set up a signal handler for the halt signal
    vm.define_handler(0x09, |vm| {
        vm.halt = true;
        Ok(())
    });

    // Load program into memory
    for (i, &byte) in program.iter().enumerate() {
        vm.memory.write(i as u16, byte);
    }

    // Run the program until it halts
    while !vm.halt {
        vm.step().expect("Failed to execute instruction");
    }

    // Check if registers contain expected values
    assert_eq!(vm.get_register(Register::A), 10);
    assert_eq!(vm.get_register(Register::B), 30);
    assert_eq!(vm.get_register(Register::C), 20);

    // The stack pointer should be back at the initial position
    assert_eq!(vm.get_register(Register::SP), 0x1000);
}

#[test]
fn test_program_add_asm() {
    // This test simulates running the program in prog/add_asm
    let mut vm = Machine::new();

    // The assembly program (simplified to the meaningful operations):
    // PUSH #10
    // PUSH #24
    // ADDS
    // POP B
    // PUSH #5
    // PUSH #22
    // ADDS
    // POP C
    // PUSH #100
    // POP A
    // SIG $09
    let program = [
        Op::Push(0).value(),
        10,
        Op::Push(0).value(),
        24,
        Op::AddStack.value(),
        0,
        Op::PopRegister(Register::B).value(),
        Register::B as u8,
        Op::Push(0).value(),
        5,
        Op::Push(0).value(),
        22,
        Op::AddStack.value(),
        0,
        Op::PopRegister(Register::C).value(),
        Register::C as u8,
        Op::Push(0).value(),
        100,
        Op::PopRegister(Register::A).value(),
        Register::A as u8,
        Op::Signal(0).value(),
        9,
    ];

    // Set up a signal handler for the halt signal
    vm.define_handler(0x09, |vm| {
        vm.halt = true;
        Ok(())
    });

    // Load program into memory
    for (i, &byte) in program.iter().enumerate() {
        vm.memory.write(i as u16, byte);
    }

    // Run the program until it halts
    while !vm.halt {
        vm.step().expect("Failed to execute instruction");
    }

    // Check if registers contain expected values
    assert_eq!(
        vm.get_register(Register::A),
        100,
        "Register A should be 100"
    );
    assert_eq!(
        vm.get_register(Register::B),
        34,
        "Register B should be 34 (10+24)"
    );
    assert_eq!(
        vm.get_register(Register::C),
        27,
        "Register C should be 27 (5+22)"
    );
}

#[test]
fn test_stack_operation_sequence() {
    let mut vm = Machine::new();

    // Program:
    // PUSH #5
    // PUSH #10
    // PUSH #15
    // POP A     ; A = 15
    // POP B     ; B = 10
    // PUSH #20
    // PUSH #25
    // ADDS      ; 20 + 25 = 45
    // POP C     ; C = 45
    let program = [
        Op::Push(0).value(),
        5,
        Op::Push(0).value(),
        10,
        Op::Push(0).value(),
        15,
        Op::PopRegister(Register::A).value(),
        Register::A as u8,
        Op::PopRegister(Register::B).value(),
        Register::B as u8,
        Op::Push(0).value(),
        20,
        Op::Push(0).value(),
        25,
        Op::AddStack.value(),
        0,
        Op::PopRegister(Register::C).value(),
        Register::C as u8,
    ];

    // Load program into memory
    for (i, &byte) in program.iter().enumerate() {
        vm.memory.write(i as u16, byte);
    }

    // Execute all instructions
    for _ in 0..9 {
        vm.step().expect("Failed to execute instruction");
    }

    // Check register values
    assert_eq!(vm.get_register(Register::A), 15);
    assert_eq!(vm.get_register(Register::B), 10);
    assert_eq!(vm.get_register(Register::C), 45);

    // Check that one value (5) remains on the stack
    assert_eq!(vm.get_register(Register::SP), 0x1002);
    vm.registers[Register::SP as usize] -= 2; // Temporarily adjust SP to read the value
    assert_eq!(vm.memory.read2(vm.get_register(Register::SP)).unwrap(), 5);
    vm.registers[Register::SP as usize] += 2; // Restore SP
}

#[test]
fn test_memory_load_from_vec() {
    let mut vm = Machine::new();

    // Program bytes to load
    let program = vec![
        Op::Push(0).value(),
        42,
        Op::PopRegister(Register::A).value(),
        Register::A as u8,
        Op::Signal(0).value(),
        9,
    ];

    // Load program into memory
    let (bytes, instructions) = vm
        .memory
        .load_from_vec(&program, 0)
        .expect("Failed to load program");

    // Verify correct loading
    assert_eq!(bytes, 6);
    assert_eq!(instructions, 3);

    // Register halt signal handler
    vm.define_handler(0x09, |vm| {
        vm.halt = true;
        Ok(())
    });

    // Run program until halt
    while !vm.halt {
        vm.step().expect("Failed to execute instruction");
    }

    // Verify register value
    assert_eq!(vm.get_register(Register::A), 42);
}

#[test]
fn test_load_16bit_values() {
    let mut vm = Machine::new();

    // Test writing and reading 16-bit values
    vm.memory.write2(0x100, 0x1234);
    vm.memory.write2(0x102, 0xABCD);

    // Verify memory contents (little-endian format)
    assert_eq!(vm.memory.read(0x100).unwrap(), 0x34); // Low byte
    assert_eq!(vm.memory.read(0x101).unwrap(), 0x12); // High byte
    assert_eq!(vm.memory.read(0x102).unwrap(), 0xCD); // Low byte
    assert_eq!(vm.memory.read(0x103).unwrap(), 0xAB); // High byte

    // Verify 16-bit reads
    assert_eq!(vm.memory.read2(0x100).unwrap(), 0x1234);
    assert_eq!(vm.memory.read2(0x102).unwrap(), 0xABCD);
}

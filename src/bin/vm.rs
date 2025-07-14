use rustyvm::Machine;

fn main() -> Result<(), String> {
    let mut vm = Machine::new();

    vm.memory.write(0, 0xF);

    let _ = vm.step();
    vm.step()
}

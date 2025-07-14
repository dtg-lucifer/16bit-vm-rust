use rustyvm::Machine;

fn main() -> () {
    let mut vm = Machine::new();
    let _ = vm.step();
}

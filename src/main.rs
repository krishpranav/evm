mod vm;

use vm::{ExecResult, MiniVm};

fn main() {
    let code = vec![0x60, 0x03, 0x60, 0x05, 0x01, 0x00];

    let mut vm = MiniVm::new(code);
    let result = vm.run();

    println!("Result: {:?}", result);
    println!("Top of stack: {:?}", vm.peek());
}
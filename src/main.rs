use ev3c::args;
use std::env;

use ev3asm::assemble;

fn main() {
    let arguments = args::check(args::parse(env::args()));
    
    let bytecode = assemble![
    mov rax
    ];
}

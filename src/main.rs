use ev3c::args;
use std::env;

fn main() {
    let arguments = args::check(args::parse(env::args()));
}

mod elf;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let header = elf::header(file);
    println!("{}", header);
}

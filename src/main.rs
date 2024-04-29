mod structs;
mod filereader;

use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if (args.len() != 2) {
        panic!("Wrong arguments: {args:?}, expected 1 for path got {}",
            args.len() - 1);
    }
    let path = args[1].as_str();
    let (meta, map) = filereader::filereader::load_map_data_from_file(path);
    println!("Rover Map: {map:?}");
    println!("-------------------------");
    println!("Rover Metadata: {meta:?}");

}

mod structs;
mod filereader;

use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let path = match args.len() {
        1 => "map.txt",
        2 => args[1].as_str(),
        _ => panic!("Wrong arguments: {args:?}, expected 1 for path got {}",
        args.len() - 1)
    };
    let (meta, map) = filereader::load_map_data_from_file(path);
    println!("Rover Map: {map:?}");
    println!("-------------------------");
    println!("Rover Metadata: {meta:?}");

}

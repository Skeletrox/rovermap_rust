mod structs;
mod filereader;
fn main() {
    println!("Hello, world!");
    let path = "map.txt";
    let (meta, map) = filereader::filereader::load_map_data_from_file(path);
    println!("Rover Map: {map:?}");
    println!("-------------------------");
    println!("Rover Metadata: {meta:?}");

}

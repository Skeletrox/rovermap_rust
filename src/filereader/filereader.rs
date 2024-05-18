// A basic file reader for reading rover data
use crate::structs as structs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[doc = r"Read all rover map data from a file. The format is as follows:
    * Line 1: Algorithm for calculating the route needed to travel
    * Line 2: Width and height of the map, expressed as 'width height'
    * Line 3: Landing coordinates, expressed as 'x y'
    * Line 4: Maximum height that the rover can traverse
    * Line 5: The number of targets, say n
    * Line 6 - 5+n: The coordinates of the targets, expressed as  'x y'\
    * Line 5+n onwards: The matrix of elevations of the map"]
pub fn load_map_data_from_file(path: &str) -> (structs::RoverMetadata, structs::RoverMap) {
    let file = File::open(path).expect("Cannot open metadata file!");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(err) => panic!("Could not parse contents of file: {:?}!", err)
    };
    
    // Read the contents of the file
    let lines: Vec<&str> = contents.split("\n").collect::<Vec<_>>();

    // Read line 1
    let algorithm = lines[0].to_string();

    // Read line 2
    let width: u32;
    let height: u32;
    {
        let width_and_height = lines[1].split(" ").collect::<Vec<_>>();
        match width_and_height[0].parse::<u32>() {
            Ok(value) => width = value,
            Err(err) => panic!("Parsing Error! {:?}", err),
        };
        match width_and_height[1].parse::<u32>() {
            Ok(value) => height = value,
            Err(err) => panic!("Parsing Error! {:?}", err),
        };
    }

    // Read line 3
    let landing_x;
    let landing_y;
    {
        let x_and_y = lines[2].split(" ").collect::<Vec<_>>();
        match x_and_y[0].parse::<u32>() {
            Ok(value) => landing_x = value,
            Err(err) => panic!("Parsing Error! {:?}", err),
        };
        match x_and_y[1].parse::<u32>() {
            Ok(value) => landing_y = value,
            Err(err) => panic!("Parsing Error! {:?}", err),
        };
    }
    let landing_point = structs::Point::new_point(landing_x, landing_y);

    // Read line 4
    let max_rover_delta_height;
    match lines[3].parse::<u32>() {
        Ok(value) => max_rover_delta_height = value,
        Err(err) => panic!("Parsing Error! {:?}", err)
    };

    // Read line 5
    let num_targets;
    match lines[4].parse::<u32>() {
        Ok(value) => num_targets = value,
        Err(err) => panic!("Parsing Error! {:?}", err)
    };

    // Read line 6 - 5+n
    let mut target_sites : Vec<structs::Point> = Vec::new();
    for i in 0..num_targets {
        let landing_x;
        let landing_y;
        {
            let curr_index = usize::try_from(6+i).unwrap();
            let curr_x_and_y = lines[curr_index].split(" ").collect::<Vec<_>>();
            match curr_x_and_y[0].parse::<u32>() {
                Ok(value) => landing_x = value,
                Err(err) => panic!("Parsing Error! {:?}", err)
            };
            match curr_x_and_y[1].parse::<u32>() {
                Ok(value) => landing_y = value,
                Err(err) => panic!("Parsing Error! {:?}", err)
            };
        }
        let target_site = structs::Point::new_point(landing_x, landing_y);
        // The values in landing_sites have their ownership transferred to
        // the vector
        target_sites.push(target_site);
        // landing_site = .... will not work now as ownership of the memory
        // has been transferred to the vector
    }
    
    // Read remaining lines for reading elevations
    let mut elevations : Vec<Vec<u32>> = Vec::new();
    for i in 0..height {
        let mut elevations_this_line: Vec<u32> = Vec::new();
        let i_index = usize::try_from(5 + num_targets + i).unwrap();
        let curr_height_elevations = lines[i_index].split(
            " ").collect::<Vec<_>>();
        for j in 0..width {
            let j_index = usize::try_from(j).unwrap();
            match curr_height_elevations[j_index].parse::<u32>() {
                Ok(value) => elevations_this_line.push(value),
                Err(err) => panic!("Parsing Error! {:?}", err)
            };
        }
        elevations.push(elevations_this_line);
    }
    let rover_map = structs::RoverMap::generate(width, height, landing_point, target_sites, elevations);
    let rover_meta = structs::RoverMetadata::generate(algorithm, max_rover_delta_height);
    return (rover_meta, rover_map);

}
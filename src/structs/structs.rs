// Structs that we need for implementing the rover

#[doc = 
r" A point is a simple [x,y] collection, referring to a point in space."]
#[derive(Debug)]
pub struct Point {
    // x-coordinate of point
    x: u32,
    // y-coordinate of point
    y: u32,
}

#[doc = r"A map is a 2-dimensional collection of elevations, along with landing
coordinates for the rover.
* TODO(@Skeletrox): Support multiple landing points"]
#[derive(Debug)]
pub struct RoverMap {
    // The width of the map
    width: u32,
    // The height of the map
    height: u32,
    // Landing site coordinates for the rover, denoted as [x,y]
    landing_coords: Point,
    // Target site coordinates for the rover
    target_sites: Vec<Point>,
    // Elevations, described as a 2D matrix
    elevations: Vec<Vec<u32>>,
}

#[doc = r"Metadata for the rover - mainly useful for quick packing and
unpacking of rover metadata"]
#[derive(Debug)]
pub struct RoverMetadata {
    // The algorithm the rover should use
    algorithm: String,
    //  The maximum change in elevation that the rover can traverse
    elevation_delta: u32,
}

#[doc = r#"A node is a point that is visited as part of a path. It contains the
"parent" node, and the cost to reach it from the parent. The lifetime param
indicates that the node has to have the same lifetime as the parent it is
referencing."#]
#[derive(Debug)]
pub struct Node<'common> {
    // The point this node refers to
    location: Point,
    // The parent is where we came from
    parent: Option<&'common Node<'common>>,
    // Cost it would take to reach here
    cost: f32,
}

#[doc = r"* Implementations for the point"]
impl Point {
    pub fn new_point(x: u32, y: u32) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn get_coords(&self) -> [u32; 2] {
        [self.x, self.y]
    }
}

#[doc = r"* Implementations for the node"]
impl<'common> Node<'common> {

    // The first node in a node queue
    pub fn generate_genesis(location: Point) -> Node<'common> {
        Node {
            location: location,
            parent: None,
            cost: 0.0,
        }
    }

    // Any node that has a parent node defined for it
    pub fn new_node(parent: &'common Node<'common>, location: Point) -> Node<'common> {
        let parent_coords = parent.get_coords();
        let current_coords = location.get_coords();
        let mut cost_delta: f32 = (
            (current_coords[0] - parent_coords[0]).pow(2) + 
            (current_coords[1] - parent_coords[1]).pow(2)) as f32;
        cost_delta = cost_delta.sqrt();
        Node {
            location: location,
            parent: Some(parent),
            cost: parent.get_cost() + cost_delta
        }
    }

    // Get the coordinates of this Node
    pub fn get_coords(&self) -> [u32; 2] {
        self.location.get_coords()
    }

    // Get the cost of this node
    pub fn get_cost(&self) -> f32 {
        self.cost
    }
} // impl<' > Node <' >

impl RoverMap {
    pub fn generate(width: u32, height: u32, landing_coords: Point,
        target_sites: Vec<Point>,
        elevations: Vec<Vec<u32>>) -> RoverMap {
        RoverMap {
            width: width,
            height: height,
            landing_coords: landing_coords,
            target_sites: target_sites,
            elevations: elevations
        }
    }
}

impl RoverMetadata {
    pub fn generate(algorithm: String, elevation_delta: u32) -> RoverMetadata {
        RoverMetadata {
            algorithm: algorithm,
            elevation_delta: elevation_delta
        }
    }
}
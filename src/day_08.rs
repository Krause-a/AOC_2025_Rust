#![allow(unused, dead_code)]
use std::{collections::{BinaryHeap, HashMap, HashSet}, fmt::Debug, hash::Hash, str::FromStr, string::ParseError};

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let desired_connections = if test_data.is_test() {10} else {1000};
    let mut junctions : Vec<Junction> = Vec::new();
    
    for line in test_data.get_lines().unwrap() {
        let coord = Point3::from_str(&line).unwrap();
        junctions.push(Junction::new(coord));
    }

    // TODO: Make this a min heap based on connection distances
    let mut connections : BinaryHeap<Connection> = BinaryHeap::new();
    for (i, j_1) in junctions.iter().enumerate() {
        for j_2 in junctions.iter().skip(i) {
            if j_1 == j_2 {
                continue;
            }
            connections.push(Connection::new(
                    j_1,
                    j_2
            ));
        }
    }

    let mut graph = graph::Graph::new();
    let mut coord_to_index : HashMap<Point3, usize> = HashMap::new();
    for i in 0..desired_connections {
        let connection = connections.pop().unwrap();
        log::debug(|| format!("Checking connection: {:?}", connection));
        let j_1_index;
        let mut both_existing = true;
        if let Some(&existing_index) = coord_to_index.get(&connection.junc_1.v) {
            j_1_index = existing_index;
        }
        else {
            j_1_index = graph.add_node(connection.junc_1.v);
            coord_to_index.insert(connection.junc_1.v, j_1_index);
            both_existing = false;
        }
        let j_2_index;
        if let Some(&existing_index) = coord_to_index.get(&connection.junc_2.v) {
            j_2_index = existing_index;
        }
        else {
            j_2_index = graph.add_node(connection.junc_2.v);
            coord_to_index.insert(connection.junc_2.v, j_2_index);
            both_existing = false;
        }
        if both_existing {
            log::debug(|| format!("Both alread existed!"));
        }
        graph.add_edge(j_1_index, j_2_index);
    }

    let mut webs : Vec<HashSet<Point3>> = Vec::new();

    let mut connection_counts = Vec::new();
    for node in coord_to_index.iter().map(|(_, i)| graph.get_node(*i).unwrap()) {
        log::debug(|| format!("{:?}", node));
        if webs.iter().all(|web| !web.contains(&node.value)) {
            let mut web = HashSet::new();
            connection_counts.push(count_connections_into(&graph, node, &mut web));
            webs.push(web);
        }
    }
    connection_counts.sort();
    let top_3_connections = connection_counts.into_iter().take(3);
    let mult_total : usize = top_3_connections.reduce(|acc, v| acc * v).unwrap();

    return mult_total.to_string();
}

fn count_connections_into(g: &graph::Graph<Point3>, n: &graph::Node<Point3>, s: &mut HashSet<Point3>) -> usize {
    if s.contains(&n.value) {
        return 0;
    }
    s.insert(n.value.clone());
    let mut connections = 1;
    for neighbor_index in n.neighbors.iter() {
        let neighbor = g.get_node(*neighbor_index).unwrap();
        connections += count_connections_into(g, neighbor, s);
    }
    return connections;
}

pub fn part_2(test_data: TestData) -> String {
    return String::from("Wowies Day 8 Part 2");
}

mod graph {
    #[derive(Debug)]
    pub struct Graph<T> {
        nodes: Vec<Node<T>>,
    }

    #[derive(Debug)]
    pub struct Node<T>
        where T: Sized
    {
        pub value: T,
        pub neighbors: Vec<usize>,
    }

    impl<T> Node<T> {
        pub fn new(v: T) -> Node<T> {
            Node { value: v, neighbors: Vec::new() }
        }
    }

    impl<T> Graph<T> {
        pub fn new() -> Graph<T> {
            Graph { nodes: Vec::new() }
        }
        pub fn add_node(self: &mut Self, v: T) -> usize {
            let id = self.nodes.len();
            self.nodes.push(Node::new(v));
            id
        }
        pub fn add_edge(self: &mut Self, a: usize, b: usize) {
            self.nodes[a].neighbors.push(b);
            self.nodes[b].neighbors.push(a);
        }
        pub fn get_node(self: &Self, index: usize) -> Option<&Node<T>> {
            self.nodes.get(index)
        }
    }

}


// TODO: Consider moving into utility file
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 {
            x,
            y,
            z,
        }
    }

    fn distance(self: &Self, other: &Self) -> f32 {
        let d_x = (self.x - other.x).abs();
        let d_y = (self.y - other.y).abs();
        let d_z = (self.z - other.z).abs();
        (d_x.powi(2) + d_y.powi(2) + d_z.powi(2)).sqrt()
    }
}

impl Eq for Point3 {}

impl Hash for Point3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.x.to_bits());
        state.write_u32(self.y.to_bits());
        state.write_u32(self.z.to_bits());
    }
}

#[cfg(test)]
mod test_point_3 {
    use std::hash::Hasher;

    use super::*;

    #[test]
    fn test_point_hash() {
        let collection = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.2345, 0.0),
            Point3::new(100.001, 0.0, 0.0),
            Point3::new(100.0, 0.0, 0.0),
            Point3::new(0.0, 1.2343, 0.0),
            Point3::new(162.0, 817.0, 812.0),
            Point3::new(425.0, 690.0, 689.0),
            Point3::new(162.0, 817.0, 812.0),
            Point3::new(431.0, 825.0, 988.0),
            Point3::new(906.0, 360.0, 560.0),
            Point3::new(805.0, 96.0, 715.0),
            Point3::new(425.0, 690.0, 689.0),
            Point3::new(431.0, 825.0, 988.0),
            Point3::new(862.0, 61.0, 35.0),
            Point3::new(984.0, 92.0, 344.0),
        ];


        for p1 in collection.iter() {
            let mut p1_hasher = std::hash::DefaultHasher::new();
            p1.hash(&mut p1_hasher);
            let p1_hash = p1_hasher.finish();
            for p2 in collection.iter() {
                let mut p2_hasher = std::hash::DefaultHasher::new();
                let is_eq = p1.eq(p2);
                let is_strict_eq = p1.x == p2.x && p1.y == p2.y && p1.z == p2.z;
                assert_eq!(is_eq, is_strict_eq);
                p2.hash(&mut p2_hasher);
                let p2_hash = p2_hasher.finish();
                if is_eq {
                    assert_eq!(p1_hash, p2_hash);
                }
                else {
                    assert_ne!(p1_hash, p2_hash);
                }
            }
        }
    }
}

struct ParsePoint3Error {}
impl Debug for ParsePoint3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse string into Point3")
    }
}

impl FromStr for Point3 {
    type Err = ParsePoint3Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits : Vec<_> = s.split(",").collect();
        if splits.len() != 2 && splits.len() != 3 {
            Err(ParsePoint3Error {})
        }
        else {
            let x = splits[0].parse().map_err(|_| ParsePoint3Error{})?;
            let y = splits[1].parse().map_err(|_| ParsePoint3Error{})?;
            let z = splits.get(2).map(|s| s.parse().map_err(|_| ParsePoint3Error{})).unwrap_or(Ok(0.0))?;
            Ok(Point3 {
                x,
                y,
                z,
            })
        }
    }
}

#[derive(Debug)]
struct Junction {
    v: Point3,
}

impl Junction {
    fn new(v: Point3) -> Junction {
        Junction {
            v,
        }
    }
}

impl PartialEq for Junction {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

#[derive(Debug)]
struct Connection<'a> {
    junc_1: &'a Junction,
    junc_2: &'a Junction,
    distance: f32,
}

impl Connection <'_> {
    fn new<'a>(j_1: &'a Junction, j_2: &'a Junction) -> Connection<'a> {
        Connection {
            junc_1: j_1,
            junc_2: j_2,
            distance: j_1.v.distance(&j_2.v)
        }
    }
}

impl PartialEq for Connection <'_> {
    fn eq(&self, other: &Self) -> bool {
        self.junc_1.eq(&other.junc_1) &&
        self.junc_2.eq(&other.junc_2)
    }
}
impl Eq for Connection <'_> {}

// Comparrisons are reversed to make it act as a min heap when used in a heap.
impl PartialOrd for Connection <'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Connection <'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.total_cmp(&self.distance)
    }
}

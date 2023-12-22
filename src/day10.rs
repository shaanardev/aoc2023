use std::collections::{HashMap, HashSet, VecDeque};

use itertools::iproduct;

use crate::Solution;

#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: i32,
    y: i32
}
impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord {
            x,
            y
        }
    }

    fn get_neighbors(&self) -> Vec<Coord> {
        iproduct!(-1..=1, -1..=1)
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0) )
            .map(|(dx, dy)| Coord {
                x: self.x + dx,
                y: self.y + dy,
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Node {
    coord: Coord,
    data: char
}
impl Node {
    pub fn get_valid_connections(&self, map: &HashMap<Coord, Node>) -> Vec<Coord> {
        //x goes from left to right, y goes from top to bottom
        match self.get_node_data(map) {
            '|' => vec![
                Coord::new(self.coord.x, self.coord.y + 1),
                Coord::new(self.coord.x, self.coord.y - 1),
            ],
            '-' => vec![
                Coord::new(self.coord.x + 1, self.coord.y),
                Coord::new(self.coord.x - 1, self.coord.y),
            ],
            'L' => vec![
                Coord::new(self.coord.x, self.coord.y - 1),
                Coord::new(self.coord.x + 1, self.coord.y),
            ],
            'J' => vec![
                Coord::new(self.coord.x, self.coord.y - 1),
                Coord::new(self.coord.x - 1, self.coord.y),
            ],
            '7' => vec![
                Coord::new(self.coord.x, self.coord.y + 1),
                Coord::new(self.coord.x - 1, self.coord.y),
            ],
            'F' => vec![
                Coord::new(self.coord.x, self.coord.y + 1),
                Coord::new(self.coord.x + 1, self.coord.y),
            ],
            _ => panic!("Error no connected coordinates for this node {:?}", self)
        }
    }

    fn get_node_data(&self, map: &HashMap<Coord, Node>) -> char {
        match self.data {
            'S' => self.determine_node_data(map),
            _ => self.data,
        }
    }

    fn determine_node_data(&self, map: &HashMap<Coord, Node>) -> char {
        let neighbors = self.coord.get_neighbors();
        let mut connected_coords = vec![];
        
        connected_coords.extend(neighbors.into_iter().filter_map(|neighbor| {
            map.get(&neighbor)
                .and_then(|node| {
                    let connections = node.get_valid_connections(map);
                    if connections.contains(&self.coord) {
                        Some(neighbor)
                    } else {
                        None
                    }
                })
        }));

        let valid_chars = vec!['|', '-', 'L', 'J', '7', 'F'];
        let node = valid_chars
                    .into_iter()
                    .map(|c| Node {
                        coord: self.coord.clone(),
                        data: c,
                    })
                    .filter(|node| {
                        let mut coords = node.get_valid_connections(map).clone();
                        coords.sort();
                        coords == connected_coords
                    })
                    .next()
                    .unwrap();
        node.data
    }
}


#[derive(Debug)]
pub struct Day10;
impl Solution for Day10 {
    type ParsedInput = HashMap<Coord, Node>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let lines = input.lines().collect::<Vec<&str>>();
        let grid = lines
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        
        let mut map = HashMap::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, &data) in row.iter().enumerate() {
                let coord = Coord::new(x as i32, y as i32);
                map.insert(coord.clone(), Node { coord, data });
            }
        }

        map
    }

    fn part_one(map: &mut Self::ParsedInput) -> String {
        let start_node = get_start_node(map);

        let visited = breadth_first_traversal(start_node, &map);
        visited.values().max().unwrap().to_string()
    }

    fn part_two(map: &mut Self::ParsedInput) -> String {
        let start_node = get_start_node(map);
        let traversed = breadth_first_traversal(start_node, &map);
        let found_loop = traversed.into_keys().collect::<HashSet<Node>>();
        let nodes_in_loop = map
                                        .values()
                                        .clone()
                                        .filter(|&node| !found_loop.contains(node))
                                        .filter(|&node| point_in_polygon(node, &found_loop, &map))
                                        .collect::<Vec<&Node>>(); 
        nodes_in_loop.len().to_string()
    }
}

fn get_start_node(map: &HashMap<Coord, Node>) -> Node {
    map
        .values()
        .filter(|&node| node.data == 'S')
        .next()
        .unwrap()
        .clone()
}

fn breadth_first_traversal(starting_node: Node, map: &HashMap<Coord, Node>) -> HashMap<Node, i32> {
    let mut visited = HashMap::new();
    visited.insert(starting_node.clone(), 0);

    let mut current_nodes = vec![starting_node];

    let mut steps = 0;

    while !current_nodes.is_empty() {
        steps += 1;

        let next_nodes = get_next_nodes(&current_nodes, map);
        
        let next_nodes: Vec<Node> = next_nodes
           .into_iter()
           .filter_map(|node| {
               if !visited.contains_key(&node) || *visited.get(&node).unwrap() > steps {
                   Some(node.clone())
               } else {
                   None
               }
           })
           .collect();
        
        current_nodes = next_nodes;
        
        for node in current_nodes.iter() {
           visited.insert(node.clone(), steps);
        }
    }
    visited
}

fn get_next_nodes(current_nodes: &[Node], map: &HashMap<Coord, Node>) -> Vec<Node> {
    current_nodes
        .iter()
        .flat_map(|node| node.get_valid_connections(map))
        .filter_map(|coord| map.get(&coord).cloned())
        .collect()
}

//https://en.wikipedia.org/wiki/Point_in_polygon
fn point_in_polygon(node: &Node, found_loop: &HashSet<Node>, map: &HashMap<Coord, Node>) -> bool {
    let mut current_coord = node.coord.clone();
    let mut intersections = 0;

    while map.contains_key(&current_coord) {
        let current_node = map.get(&current_coord).unwrap();
        let data = current_node.get_node_data(map);
     
        if found_loop.contains(current_node) && ['|', 'J', 'L'].contains(&data) {
            intersections += 1;
        }
     
        current_coord.x += 1;
     }
     
     (intersections % 2) == 1
}
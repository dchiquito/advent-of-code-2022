use std::collections::HashMap;
use std::ops::Add;

use crate::advent;
use regex::Regex;

#[derive(Debug, Clone)]
struct Room {
    id: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl Room {
    fn is_important(&self) -> bool {
        self.flow > 0 || self.id == "AA"
    }
}

fn parse_rooms() -> HashMap<String, Room> {
    let lines = advent::read_input(16);
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z, ]+)",
    )
    .unwrap();
    let mut rooms = HashMap::new();
    lines
        .iter()
        .map(|line| {
            let capture = re.captures(line).unwrap();
            Room {
                id: capture[1].into(),
                flow: capture[2].parse().unwrap(),
                tunnels: capture[3].split(", ").map(str::to_string).collect(),
            }
        })
        .for_each(|room| {
            rooms.insert(room.id.to_string(), room.clone());
        });
    rooms
}

#[derive(Debug)]
struct Node {
    id: String,
    flow: u32,
    edges: HashMap<String, u32>,
}

impl Node {
    fn new(room: &Room) -> Node {
        Node {
            id: room.id.clone(),
            flow: room.flow,
            edges: HashMap::new(),
        }
    }
    fn calculate_distances(&mut self, rooms: &HashMap<String, Room>) {
        fn visit(
            rooms: &HashMap<String, Room>,
            id: &str,
            distance: u32,
            distances: &mut HashMap<String, u32>,
        ) {
            let room = rooms.get(id).unwrap();
            if let Some(previous_distance) = distances.get(id) {
                if &distance >= previous_distance {
                    return;
                }
            }
            distances.insert(id.to_string(), distance);
            for adjacent in room.tunnels.iter() {
                visit(rooms, adjacent, distance + 1, distances);
            }
        }
        let mut distances = HashMap::<String, u32>::new();
        visit(rooms, &self.id, 0, &mut distances);
        for (id, room) in rooms.iter() {
            if room.flow > 0 && id != &self.id {
                self.edges
                    .insert(id.to_string(), *distances.get(id).unwrap());
            }
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn new(rooms: &HashMap<String, Room>) -> Graph {
        let mut graph = Graph {
            nodes: HashMap::new(),
        };
        for (_, room) in rooms.iter() {
            if room.is_important() {
                graph.nodes.insert(room.id.clone(), Node::new(room));
            }
        }
        for (_id, node) in graph.nodes.iter_mut() {
            node.calculate_distances(rooms);
        }
        graph
    }
}

fn solve_1(graph: &Graph, time_limit: u32) {
    for (_, node) in graph.nodes.iter() {
        println!("{} {} {:?}", node.id, node.flow, node.edges);
    }
    let mut visited = vec![];
    fn visit(
        graph: &Graph,
        id: &str,
        time: u32,
        time_limit: u32,
        visited: &mut Vec<String>,
    ) -> u32 {
        if time > time_limit {
            return 0;
        }
        visited.push(id.to_string());
        let flow_per_tick = visited
            .iter()
            .map(|i| graph.nodes.get(i).unwrap().flow)
            .reduce(u32::add)
            .unwrap_or(0);
        let node = graph.nodes.get(id).unwrap();

        // default is just waiting until time expires
        let mut max_flow = (time_limit - time) * flow_per_tick;
        for (adj_id, distance) in node.edges.iter() {
            if visited.contains(adj_id) {
                // already visited that node, can't turn it on again
                continue;
            }
            let flow;
            if distance + 1 >= time_limit - time {
                // it would take too long to turn it on
                flow = flow_per_tick * (time_limit - time);
            } else {
                // flow while we walk there and turn on the valve + recursion
                flow = (flow_per_tick * (distance + 1))
                    + visit(graph, adj_id, time + distance + 1, time_limit, visited);
            }
            if flow > max_flow {
                max_flow = flow;
            }
        }
        visited.pop();
        max_flow
    }
    println!("{}", visit(graph, "AA", 0, time_limit, &mut visited));
}

pub fn solve() {
    let rooms = parse_rooms();
    let graph = Graph::new(&rooms);
    solve_1(&graph, 30);
}

use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, VecDeque, HashSet}};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day16/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let graph_lines = lines.map(|x| parse_graph_line(&x.unwrap()));
    let graph = parse_graph(graph_lines);

    let complete_graph = get_complete_graph(&graph);

    let best_score = find_best_score_dfs(&complete_graph, State {
        activated_vertices: HashSet::new(),
        current_vertex: "AA".to_string(),
        minutes_remaining: 30,
    });

    println!("best score: {}", best_score);
}

#[derive(Debug)]
struct Vertex {
    flow_rate: i32,
    neighbours: Vec<String>,
}

#[derive(Debug)]
struct Graph {
    vertices: HashMap<String, Vertex>,
}

#[derive(Debug)]
struct GraphLine {
    name: String,
    flow_rate: i32,
    neighbours: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    current_vertex: String,
    activated_vertices: HashSet<String>,
    minutes_remaining: i32,
}

#[derive(Debug)]
struct Entry {
    state: State,
    score: i32,
}

#[derive(Debug)]
struct WeightedVertex {
    flow_rate: i32,
    neighbours: Vec<(String, i32)>,
}

#[derive(Debug)]
struct WeightedGraph {
    vertices: HashMap<String, WeightedVertex>,
}

fn get_neighbour_distances(graph: &Graph, vertex: &str) -> HashMap<String, i32> {
    let mut visited = HashMap::<String, i32>::new();
    let mut queue = VecDeque::<(i32, String)>::new();

    queue.push_back((0, vertex.to_string()));
    
    while let Some((cost, vert)) = queue.pop_front() {
        visited.insert(vert.clone(), cost);

        for succ in &graph.vertices[&vert].neighbours {
            if visited.contains_key(succ) {
                continue;
            }
            queue.push_back((cost+1, succ.clone()));
        }
    }

    visited
}

fn get_complete_graph(graph: &Graph) -> WeightedGraph {
    let vertices = graph.vertices.iter().map(|(k, v)| {
        let distances = get_neighbour_distances(graph, k);
        let vertex = WeightedVertex {
            flow_rate: v.flow_rate,
            // Takes 1 minute to activate the thing so +1 on top of travel distance
            neighbours: graph.vertices.keys().map(|s| (s.clone(), distances[s] + 1)).collect(),
        };
        (k.clone(), vertex)
    });

    WeightedGraph { vertices: vertices.collect() }
}

fn get_successors_complete(graph: &WeightedGraph, e: &Entry) -> Vec<Entry> {
    let mut vec = Vec::new();

    if e.state.minutes_remaining == 0 {
        return vec;
    }

    let current_vertex = &graph.vertices[&e.state.current_vertex];

    for (neighbour, cost) in &current_vertex.neighbours {
        if e.state.activated_vertices.contains(neighbour) {
            continue;
        }
        let neighbour_vertex = &graph.vertices[neighbour];
        if neighbour_vertex.flow_rate == 0 {
            continue;
        }
        let new_minutes_remaining = e.state.minutes_remaining - cost;
        if new_minutes_remaining <= 0 {
            continue;
        }
        let mut new_activated_vertices = e.state.activated_vertices.clone();
        new_activated_vertices.insert(neighbour.clone());
        vec.push(Entry {
            score: e.score + (neighbour_vertex.flow_rate * new_minutes_remaining),
            state: State {
                current_vertex: neighbour.clone(),
                activated_vertices: new_activated_vertices,
                minutes_remaining: new_minutes_remaining,
            }
        })
    }

    vec
}

/// Always overestimates.
fn get_remaining_potential_score_complete(graph: &WeightedGraph, state: &State) -> i32 {
    let mut remaining_valve_scores = graph.vertices.iter()
        .filter(|&(k, _)| !state.activated_vertices.contains(k))
        .map(|(_, v)| v.flow_rate).collect::<Vec<_>>();
    remaining_valve_scores.sort();
    remaining_valve_scores.reverse();

    (0..).map(|x| state.minutes_remaining - (2*x)).take_while(|x| *x > 0)
    .zip(remaining_valve_scores.into_iter())
    .map(|(t, s)| t * s).sum::<i32>()
}

fn find_best_score_dfs(graph: &WeightedGraph, initial_state: State) -> i32 {
    let mut queue = VecDeque::<(i32, Entry)>::new();

    queue.push_back((get_remaining_potential_score_complete(graph, &initial_state), Entry{state: initial_state, score: 0}));

    let mut best_seen_score = 0;

    loop {
        match queue.pop_back() {
            None => { return best_seen_score; }
            Some((s, candidate)) => {
                if best_seen_score > s {
                    continue;
                }

                best_seen_score = best_seen_score.max(candidate.score);


                let mut successors = get_successors_complete(graph, &candidate).into_iter().map(|succ| {
                    let potential_score = succ.score + get_remaining_potential_score_complete(graph, &succ.state);
                    (potential_score, succ)
                }).collect::<Vec<_>>();
                successors.sort_by_key(|x| x.1.score);

                queue.extend(successors);
            }
        }
    }
}

fn parse_graph<T: Iterator<Item=GraphLine>>(lines: T) -> Graph {
    let mut m = HashMap::new();
    for line in lines {
        m.insert(line.name, Vertex{flow_rate: line.flow_rate, neighbours: line.neighbours});
    }
    Graph { vertices: m }
}

lazy_static! {
    static ref GRAPH_LINE_REGEX: Regex = Regex::new(r"^Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
}
fn parse_graph_line(line: &str) -> GraphLine {
    let captures = GRAPH_LINE_REGEX.captures(line).unwrap();

    GraphLine {
        name: captures[1].to_string(),
        flow_rate: captures[2].parse().unwrap(),
        neighbours: captures[3].split(", ").map(|s| s.to_string()).collect(),
    }
}
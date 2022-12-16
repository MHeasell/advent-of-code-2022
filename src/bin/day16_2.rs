use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, VecDeque, BTreeSet}, mem::swap};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day16/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let graph_lines = lines.map(|x| parse_graph_line(&x.unwrap()));
    let graph = parse_graph(graph_lines);

    let complete_graph = get_complete_graph(&graph, decode_vertex_name("AA"));

    let best_score = find_best_score_rec_wrapper(&complete_graph, &State {
        activated_vertices: BTreeSet::new(),
        me: ActorState{
            current_vertex: decode_vertex_name("AA"),
            minutes_remaining: 26,
        },
        elephant: ActorState{
            current_vertex: decode_vertex_name("AA"),
            minutes_remaining: 26,
        },
    });

    println!("best score: {}", best_score);
}

#[derive(Debug)]
struct Vertex {
    flow_rate: i32,
    neighbours: Vec<VertexId>,
}

#[derive(Debug)]
struct Graph {
    vertices: HashMap<VertexId, Vertex>,
}

#[derive(Debug)]
struct GraphLine {
    name: String,
    flow_rate: i32,
    neighbours: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct ActorState {
    current_vertex: VertexId,
    minutes_remaining: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    me: ActorState,
    elephant: ActorState,
    activated_vertices: BTreeSet<VertexId>,
}

#[derive(Debug)]
struct Entry {
    state: State,
    score: i32,
}

#[derive(Debug)]
struct WeightedVertex {
    flow_rate: i32,
    neighbours: Vec<(VertexId, i32)>,
}

#[derive(Debug)]
struct WeightedGraph {
    vertices: HashMap<VertexId, WeightedVertex>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct VertexId(i32);

fn decode_vertex_name(name: &str) -> VertexId {
    let bytes = name.as_bytes();
    let first = i32::from(bytes[0] - b'A');
    let second = i32::from(bytes[1] - b'A');
    VertexId((first * 26) + second)
}

fn get_neighbour_distances(graph: &Graph, vertex: VertexId) -> HashMap<VertexId, i32> {
    let mut visited = HashMap::<VertexId, i32>::new();
    let mut queue = VecDeque::<(i32, VertexId)>::new();

    queue.push_back((0, vertex));
    
    while let Some((cost, vert)) = queue.pop_front() {
        visited.insert(vert, cost);

        for succ in &graph.vertices[&vert].neighbours {
            if visited.contains_key(succ) {
                continue;
            }
            queue.push_back((cost+1, *succ));
        }
    }

    visited
}

fn get_complete_graph(graph: &Graph, initial_vertex: VertexId) -> WeightedGraph {
    let vertices = graph.vertices.iter()
    .filter(|&(k, v)| v.flow_rate > 0 || *k == initial_vertex)
    .map(|(k, v)| {
        let distances = get_neighbour_distances(graph, *k);
        let vertex = WeightedVertex {
            flow_rate: v.flow_rate,
            // Takes 1 minute to activate the thing so +1 on top of travel distance
            neighbours: graph.vertices
                .iter()
                .filter(|&(k, v)| v.flow_rate > 0 || *k == initial_vertex)
                .map(|(k, _)| (k.clone(), distances[k] + 1))
                .collect(),
        };
        (k.clone(), vertex)
    });

    WeightedGraph { vertices: vertices.collect() }
}

fn get_successors_complete(graph: &WeightedGraph, e: &Entry) -> Vec<Entry> {
    let mut vec = Vec::new();

    let me_current_vertex = &graph.vertices[&e.state.me.current_vertex];
    let elephant_current_vertex = &graph.vertices[&e.state.elephant.current_vertex];


    // new states if i move
    for (neighbour, cost) in &me_current_vertex.neighbours {
        if e.state.activated_vertices.contains(neighbour) {
            continue;
        }
        let neighbour_vertex = &graph.vertices[neighbour];

        let new_minutes_remaining = e.state.me.minutes_remaining - cost;
        if new_minutes_remaining <= 0 {
            continue;
        }
        let mut new_activated_vertices = e.state.activated_vertices.clone();
        new_activated_vertices.insert(neighbour.clone());
        vec.push(Entry {
            score: e.score + (neighbour_vertex.flow_rate * new_minutes_remaining),
            state: State {
                activated_vertices: new_activated_vertices,
                me: ActorState {
                    current_vertex: neighbour.clone(),
                    minutes_remaining: new_minutes_remaining
                },
                elephant: e.state.elephant.clone(),
            }
        })
    }

    // new states if elephant moves
    for (neighbour, cost) in &elephant_current_vertex.neighbours {
        if e.state.activated_vertices.contains(neighbour) {
            continue;
        }
        let neighbour_vertex = &graph.vertices[neighbour];

        let new_minutes_remaining = e.state.elephant.minutes_remaining - cost;
        if new_minutes_remaining <= 0 {
            continue;
        }
        let mut new_activated_vertices = e.state.activated_vertices.clone();
        new_activated_vertices.insert(neighbour.clone());
        vec.push(Entry {
            score: e.score + (neighbour_vertex.flow_rate * new_minutes_remaining),
            state: State {
                activated_vertices: new_activated_vertices,
                me: e.state.me.clone(),
                elephant: ActorState {
                    current_vertex: neighbour.clone(),
                    minutes_remaining: new_minutes_remaining,
                },
            }
        })
    }

    // try to normalize the states by keeping the actors in order
    for s in &mut vec {
        if s.state.elephant < s.state.me {
            swap(&mut s.state.me, &mut s.state.elephant);
        }
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

    let mut sum = 0;

    // It will take at least 1 minute to do anything.
    let mut me_remaining_time = state.me.minutes_remaining - 1;
    let mut elephant_remaining_time = state.elephant.minutes_remaining - 1;

    for score in remaining_valve_scores.into_iter() {
        if me_remaining_time <= 0 || elephant_remaining_time <= 0 {
            break;
        }
        if me_remaining_time > elephant_remaining_time {
            sum += me_remaining_time * score;

            // takes 2 minutes to move to another room from here and do something else
            me_remaining_time -= 2;
        }
        else {
            sum += elephant_remaining_time * score;

            // takes 2 minutes to move to another room from here and do something else
            elephant_remaining_time -= 2;
        }
    }

    sum
}

fn find_best_score_rec_wrapper(graph: &WeightedGraph, initial_state: &State) -> i32 {
    let mut lookup = HashMap::new();
    find_best_score_rec(graph, initial_state, &mut lookup)
}

// manually memoized recursive func
fn find_best_score_rec(graph: &WeightedGraph, initial_state: &State, lookup: &mut HashMap<State, i32>) -> i32 {
    if let Some(score) = lookup.get(&initial_state) {
        return *score;
    }

    let mut successors = get_successors_complete(graph, &Entry {score: 0, state: initial_state.clone() });
    successors.sort_by_key(|x| x.score);

    let mut best_score = 0;
    for succ in successors.iter().rev() {
        let potential_remaining_score = get_remaining_potential_score_complete(graph, &succ.state);
        if succ.score + potential_remaining_score < best_score {
            continue;
        }

        let remaining_score = find_best_score_rec(graph, &succ.state, lookup);
        let total_score = succ.score + remaining_score;
        best_score = best_score.max(total_score);
    }

    lookup.insert(initial_state.clone(), best_score);
    best_score
}

fn parse_graph<T: Iterator<Item=GraphLine>>(lines: T) -> Graph {
    let mut m = HashMap::new();
    for line in lines {
        m.insert(
            decode_vertex_name(&line.name),
            Vertex{
                flow_rate: line.flow_rate,
                neighbours: line.neighbours.iter().map(|x| decode_vertex_name(x)).collect()
            });
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
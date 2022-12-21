use std::{fs::File, io::{BufReader, BufRead}, ops::{Add, AddAssign, Sub, SubAssign}, cmp::Ordering, collections::HashMap};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day19/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let blueprints = lines.map(|x| parse_blueprint(&x.unwrap())).collect::<Vec<_>>();

    let mut sum = 0;
    for blueprint in blueprints {
        let geodes = find_max_geodes(&blueprint);
        println!("blueprint {:?}, utility {:?}", blueprint.blueprint_id, geodes);
        sum += geodes.0 * blueprint.blueprint_id.0;
    }

    println!("sum of qualities: {}", sum);
}

const ACTIONS: [Action; 5] = [
        Action::DoNothing,
        Action::BuildOreRobot,
        Action::BuildClayRobot,
        Action::BuildObsidianRobot,
        Action::BuildGeodeRobot,
    ];

fn get_possible_actions(state: &State, blueprint: &Blueprint) -> Vec<Action> {
    if state.time_remaining <= 1 {
        return Vec::new()
    }
    ACTIONS.iter().copied().filter(|a| can_do(state, blueprint, *a)).collect()
}

fn create_initial_state() -> State {
    State {
        ore: OreQuantity(0),
        clay: ClayQuantity(0),
        obsidian: ObsidianQuantity(0),

        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,

        time_remaining: 24,
    }
}

fn find_max_geodes(blueprint: &Blueprint) -> GeodeQuantity {
    let mut lookup = HashMap::new();
    find_max_geodes_inner(&create_initial_state(), blueprint, &mut lookup)
}

fn find_max_geodes_inner(state: &State, blueprint: &Blueprint, lookup: &mut HashMap<State, GeodeQuantity>) -> GeodeQuantity {
    if let Some(geodes) = lookup.get(state) {
        return *geodes;
    }

    let mut successors = get_possible_actions(state, blueprint).into_iter().map(|a| {
        let mut s = state.clone();
        tick(&mut s, blueprint, a);
        s
    }).collect::<Vec<_>>();
    successors.sort_unstable_by(|a, b| cmp_states(a, b, blueprint));

    let geodes_yielded_this_turn = GeodeQuantity(state.geode_robots);

    let mut most_future_geodes = GeodeQuantity(0);
    for succ in successors.iter().rev() {
        let potential_future_geodes = get_max_possible_future_geodes(&succ, blueprint);
        if potential_future_geodes < most_future_geodes {
            continue;
        }

        let actual_future_geodes = find_max_geodes_inner(&succ, blueprint, lookup);
        //let total_geodes = succ.geodes + remaining_geodes;
        most_future_geodes = most_future_geodes.max(actual_future_geodes);
    }

    let total_geodes = geodes_yielded_this_turn + most_future_geodes;

    lookup.insert(state.clone(), total_geodes);
    total_geodes
}

fn get_max_possible_future_geodes(s: &State, _blueprint: &Blueprint) -> GeodeQuantity {
    let mut geodes = GeodeQuantity(0);
    let mut geode_robots = s.geode_robots;
    for _ in 0..s.time_remaining {
        geodes += GeodeQuantity(geode_robots);
        geode_robots += 1;
    }
    geodes
}

fn get_min_geodes_by_end(s: &State) -> GeodeQuantity {
    GeodeQuantity(s.geode_robots * s.time_remaining)
}

fn get_min_obsidian_by_end(s: &State) -> ObsidianQuantity {
    s.obsidian + ObsidianQuantity(s.obsidian_robots * s.time_remaining)
}

fn get_min_clay_by_end(s: &State) -> ClayQuantity {
    s.clay + ClayQuantity(s.clay_robots * s.time_remaining)
}

fn get_min_ore_by_end(s: &State) -> OreQuantity {
    s.ore + OreQuantity(s.ore_robots * s.time_remaining)
}

fn cmp_states(a: &State, b: &State, _blueprint: &Blueprint) -> Ordering {
    get_min_geodes_by_end(a).cmp(&get_min_geodes_by_end(b))
    .then_with(|| get_min_obsidian_by_end(a).cmp(&get_min_obsidian_by_end(b)))
    .then_with(|| get_min_clay_by_end(a).cmp(&get_min_clay_by_end(b)))
    .then_with(|| get_min_ore_by_end(a).cmp(&get_min_ore_by_end(b)))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    ore: OreQuantity,
    clay: ClayQuantity,
    obsidian: ObsidianQuantity,

    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,

    time_remaining: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    DoNothing,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Plan {
    actions: Vec<Action>,
}

fn can_do(state: &State, blueprint: &Blueprint, action: Action) -> bool {
    match action {
        Action::DoNothing => true,
        Action::BuildOreRobot => state.ore >= blueprint.ore_robot_cost,
        Action::BuildClayRobot => state.ore >= blueprint.clay_robot_cost,
        Action::BuildObsidianRobot => state.ore >= blueprint.obsidian_robot_cost.0 && state.clay >= blueprint.obsidian_robot_cost.1,
        Action::BuildGeodeRobot => state.ore >= blueprint.geode_robot_cost.0 && state.obsidian >= blueprint.geode_robot_cost.1,
    }
}

fn tick(state: &mut State, blueprint: &Blueprint, action: Action) {
    match action {
        Action::DoNothing => {}
        Action::BuildOreRobot => {
            state.ore -= blueprint.ore_robot_cost;
        }
        Action::BuildClayRobot => {
            state.ore -= blueprint.clay_robot_cost;
        }
        Action::BuildObsidianRobot => {
            state.ore -= blueprint.obsidian_robot_cost.0;
            state.clay -= blueprint.obsidian_robot_cost.1;
        }
        Action::BuildGeodeRobot => {
            state.ore -= blueprint.geode_robot_cost.0;
            state.obsidian -= blueprint.geode_robot_cost.1;
        }
    }

    assert!(state.ore >= OreQuantity(0));
    assert!(state.clay >= ClayQuantity(0));
    assert!(state.obsidian >= ObsidianQuantity(0));

    state.ore += OreQuantity(state.ore_robots);
    state.clay += ClayQuantity(state.clay_robots);
    state.obsidian += ObsidianQuantity(state.obsidian_robots);

    match action {
        Action::DoNothing => {}
        Action::BuildOreRobot => {
            state.ore_robots += 1;
        }
        Action::BuildClayRobot => {
            state.clay_robots += 1;
        }
        Action::BuildObsidianRobot => {
            state.obsidian_robots += 1;
        }
        Action::BuildGeodeRobot => {
            state.geode_robots += 1;
        }
    }

    state.time_remaining -= 1;
}


#[derive(Debug)]
struct Blueprint {
    blueprint_id: BlueprintId,
    ore_robot_cost: OreQuantity,
    clay_robot_cost: OreQuantity,
    obsidian_robot_cost: (OreQuantity, ClayQuantity),
    geode_robot_cost: (OreQuantity, ObsidianQuantity),
}

lazy_static! {
    static ref BLUEPRINT_REGEX: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

fn parse_blueprint(line: &str) -> Blueprint {
    let captures = BLUEPRINT_REGEX.captures(line).unwrap();

    Blueprint {
        blueprint_id: BlueprintId(captures[1].parse().unwrap()),
        ore_robot_cost: OreQuantity(captures[2].parse().unwrap()),
        clay_robot_cost: OreQuantity(captures[3].parse().unwrap()),
        obsidian_robot_cost: (OreQuantity(captures[4].parse().unwrap()), ClayQuantity(captures[5].parse().unwrap())),
        geode_robot_cost: (OreQuantity(captures[6].parse().unwrap()), ObsidianQuantity(captures[7].parse().unwrap())),
    }
}

// Lots of type spam below, nothing to see here.
// There must be a way of writing a fancy macro
// so you don't have to write all this boilerplate.

#[derive(Debug)]
struct BlueprintId(i32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Minutes(i32);
impl std::fmt::Debug for Minutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} minutes", self.0)
    }
}
impl Sub for Minutes {
    type Output = Minutes;
    fn sub(self, rhs: Self) -> Self::Output {
        Minutes(self.0 - rhs.0)
    }
}
impl SubAssign for Minutes {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct OreQuantity(i32);
impl std::fmt::Debug for OreQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ore", self.0)
    }
}
impl Add for OreQuantity {
    type Output = OreQuantity;
    fn add(self, rhs: Self) -> Self::Output {
        OreQuantity(self.0 + rhs.0)
    }
}
impl AddAssign for OreQuantity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for OreQuantity {
    type Output = OreQuantity;
    fn sub(self, rhs: Self) -> Self::Output {
        OreQuantity(self.0 - rhs.0)
    }
}
impl SubAssign for OreQuantity {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ClayQuantity(i32);
impl std::fmt::Debug for ClayQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} clay", self.0)
    }
}
impl Add for ClayQuantity {
    type Output = ClayQuantity;
    fn add(self, rhs: Self) -> Self::Output {
        ClayQuantity(self.0 + rhs.0)
    }
}
impl AddAssign for ClayQuantity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for ClayQuantity {
    type Output = ClayQuantity;
    fn sub(self, rhs: Self) -> Self::Output {
        ClayQuantity(self.0 - rhs.0)
    }
}
impl SubAssign for ClayQuantity {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ObsidianQuantity(i32);
impl std::fmt::Debug for ObsidianQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} obsidian", self.0)
    }
}
impl Add for ObsidianQuantity {
    type Output = ObsidianQuantity;
    fn add(self, rhs: Self) -> Self::Output {
        ObsidianQuantity(self.0 + rhs.0)
    }
}
impl AddAssign for ObsidianQuantity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for ObsidianQuantity {
    type Output = ObsidianQuantity;
    fn sub(self, rhs: Self) -> Self::Output {
        ObsidianQuantity(self.0 - rhs.0)
    }
}
impl SubAssign for ObsidianQuantity {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GeodeQuantity(i32);
impl std::fmt::Debug for GeodeQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} geode", self.0)
    }
}
impl Add for GeodeQuantity {
    type Output = GeodeQuantity;
    fn add(self, rhs: Self) -> Self::Output {
        GeodeQuantity(self.0 + rhs.0)
    }
}
impl AddAssign for GeodeQuantity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for GeodeQuantity {
    type Output = GeodeQuantity;
    fn sub(self, rhs: Self) -> Self::Output {
        GeodeQuantity(self.0 - rhs.0)
    }
}
impl SubAssign for GeodeQuantity {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

struct OreRobotQuantity(i32);
impl std::fmt::Debug for OreRobotQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ore robot", self.0)
    }
}

struct ClayRobotQuantity(i32);
impl std::fmt::Debug for ClayRobotQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} clay robot", self.0)
    }
}

struct ObsidianRobotQuantity(i32);
impl std::fmt::Debug for ObsidianRobotQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} obsidian robot", self.0)
    }
}

struct GeodeRobotQuantity(i32);
impl std::fmt::Debug for GeodeRobotQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} geode robot", self.0)
    }
}

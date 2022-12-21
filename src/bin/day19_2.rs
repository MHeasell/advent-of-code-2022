use std::{fs::File, io::{BufReader, BufRead}, ops::{Add, AddAssign, Sub, SubAssign}, cmp::Ordering};

use lazy_static::lazy_static;
use regex::Regex;

//const INITIAL_TIME: usize = 24;
const INITIAL_TIME: usize = 32;

fn main() {
    // let file = File::open("data/day19/sample_input.txt").unwrap();
    let file = File::open("data/day19/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let blueprints = lines.map(|x| parse_blueprint(&x.unwrap())).take(3).collect::<Vec<_>>();

    let mut product = 1;
    for blueprint in blueprints {
        let geodes = find_max_geodes(&create_initial_state(), &blueprint);
        // let geodes = find_max_geodes_by_plan_augmentation(&blueprint);
        println!("blueprint {:?}, utility {:?}", blueprint.blueprint_id, geodes);
        product *= geodes.0;
    }

    println!("product of geode counts: {}", product);
}

const ACTIONS: [Action; 5] = [
        Action::DoNothing,
        Action::BuildOreRobot,
        Action::BuildClayRobot,
        Action::BuildObsidianRobot,
        Action::BuildGeodeRobot,
    ];

fn create_initial_state() -> State {
    State {
        ore: OreQuantity(0),
        clay: ClayQuantity(0),
        obsidian: ObsidianQuantity(0),
        geodes: GeodeQuantity(0),

        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,

        time_remaining: (INITIAL_TIME as i32),
    }
}

fn can_ever_do(state: &State, action: Action) -> bool {
    match action {
        Action::DoNothing => true,
        Action::BuildOreRobot => true,
        Action::BuildClayRobot => true,
        Action::BuildObsidianRobot => state.clay_robots > 0,
        Action::BuildGeodeRobot => state.obsidian_robots > 0,
    }
}

fn try_fastforward_tick(state: &mut State, blueprint: &Blueprint, action: Action) -> bool {
    if action == Action::BuildOreRobot {
        let max_ore_robots_needed = blueprint.clay_robot_cost.max(blueprint.obsidian_robot_cost.0).max(blueprint.geode_robot_cost.0);
        if state.ore_robots >= max_ore_robots_needed.0 {
            return false;
        }
    }

    if action == Action::DoNothing {
        if state.time_remaining == 0 {
            return false;
        }
        if state.geode_robots == 0 {
            return false;
        }

        while state.time_remaining > 0 {
            tick(state, blueprint, Action::DoNothing);
        }
        return true;
    }

    if !can_ever_do(state, action) {
        return false;
    }

    while !can_do(state, blueprint, action) && state.time_remaining > 1 {
        tick(state, blueprint, Action::DoNothing);
    }

    if state.time_remaining <= 1 {
        return false;
    }

    tick(state, blueprint, action);
    true
}

fn get_successors(state: &State, blueprint: &Blueprint) -> Vec<State> {
    ACTIONS.iter().copied().filter_map(|a| {
        let mut s = state.clone();
        let result = try_fastforward_tick(&mut s, blueprint, a);
        if result { Some(s) } else { None }
    }).collect()
}

fn find_max_geodes(state: &State, blueprint: &Blueprint) -> GeodeQuantity {
    let mut successors = get_successors(state, blueprint);
    successors.sort_unstable_by(|a, b| cmp_states(a, b, blueprint));

    let mut most_geodes = state.geodes;
    for succ in successors.iter().rev() {
        let potential_geodes = get_max_possible_geodes(&succ, blueprint);
        if potential_geodes < most_geodes {
            continue;
        }

        let actual_geodes = find_max_geodes(&succ, blueprint);

        assert!(potential_geodes >= actual_geodes);

        most_geodes = most_geodes.max(actual_geodes);
    }

    most_geodes
}

fn get_max_possible_geodes(s: &State, blueprint: &Blueprint) -> GeodeQuantity {
    let mut time_available_to_make_geode_robots = s.time_remaining;

    if s.obsidian_robots == 0 {
        // we will need to make at least 1 obsidian robot first
        let time_to_get_resource_for_obsidian_robot = {
            let (ore_required, clay_required) = blueprint.obsidian_robot_cost;
            let time_to_get_enough_ore = get_max_resource_seq(s.ore.0, s.ore_robots).take_while(|o| *o < ore_required.0).count();
            let time_to_get_enough_clay = get_max_resource_seq(s.clay.0, s.clay_robots).take_while(|o| *o < clay_required.0).count();
            time_to_get_enough_clay.max(time_to_get_enough_ore)
        };
        time_available_to_make_geode_robots -= 1 + (time_to_get_resource_for_obsidian_robot as i32);
    }

    if time_available_to_make_geode_robots < 1 {
        return s.geodes;
    }

    GeodeQuantity(get_max_resource_seq(s.geodes.0, s.geode_robots).skip(1).take(time_available_to_make_geode_robots.try_into().unwrap()).last().unwrap())
}

// Sequence of total resource at the start of each turn assuming we start from X robots
// and build a new robot of that collecting type every turn
fn get_max_resource_seq(initial_resource: i32, initial_robots: i32) -> impl Iterator<Item=i32> {
    (initial_robots..).scan(initial_resource, |resource_total, robots_this_turn| {
        let current_resource = *resource_total;
        *resource_total += robots_this_turn;
        Some(current_resource)
    })
}

fn cmp_states(a: &State, b: &State, _blueprint: &Blueprint) -> Ordering {
    a.geode_robots.cmp(&b.geode_robots)
    .then(a.obsidian_robots.cmp(&b.obsidian_robots))
    .then(a.clay_robots.cmp(&b.clay_robots))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    ore: OreQuantity,
    clay: ClayQuantity,
    obsidian: ObsidianQuantity,
    geodes: GeodeQuantity,

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Plan {
    actions: Vec<Action>,
}

fn can_do(state: &State, blueprint: &Blueprint, action: Action) -> bool {
    // No point doing anything on the last turn because
    // the action will have no meaningful effect on game state.
    if state.time_remaining <= 1 && action != Action::DoNothing {
        return false;
    }

    match action {
        Action::DoNothing => true,
        Action::BuildOreRobot => state.ore >= blueprint.ore_robot_cost,
        Action::BuildClayRobot => state.ore >= blueprint.clay_robot_cost,
        Action::BuildObsidianRobot => state.ore >= blueprint.obsidian_robot_cost.0 && state.clay >= blueprint.obsidian_robot_cost.1,
        Action::BuildGeodeRobot => state.ore >= blueprint.geode_robot_cost.0 && state.obsidian >= blueprint.geode_robot_cost.1,
    }
}

fn tick(state: &mut State, blueprint: &Blueprint, action: Action) {
    assert!(state.time_remaining > 0);
    assert!(can_do(state, blueprint, action));

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
    state.geodes += GeodeQuantity(state.geode_robots);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_resource_seq() {
        {
            let vals = get_max_resource_seq(0, 1).take(5).collect::<Vec<_>>();
            assert_eq!(vals, vec![0, 1, 3, 6, 10]);
        }
        {
            let vals = get_max_resource_seq(5, 1).take(5).collect::<Vec<_>>();
            assert_eq!(vals, vec![5, 6, 8, 11, 15]);
        }
        {
            let vals = get_max_resource_seq(5, 1).take(5).collect::<Vec<_>>();
            assert_eq!(vals, vec![5, 6, 8, 11, 15]);
        }
    }
}
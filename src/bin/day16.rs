use std::cmp::max;
use std::collections::HashMap;

#[test]
fn test() {
    let input = include_str!("day16_test.txt");
    assert_eq!(solve_part1(input), 1651);
    assert_eq!(solve_part2(input), 1707);
}

fn main() {
    let input = include_str!("day16_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

struct Valve {
    connections: Vec<u16>,
    flow_rate: u64,
    id: u16,
}

fn valve_id_from_name(valve_name: &str) -> u16 {
    let bytes = valve_name.as_bytes();

    let byte_1 = bytes[0] - 'A' as u8;
    let byte_2 = bytes[1] - 'A' as u8;

    return ((byte_1 as u16) << 8) | (byte_2 as u16);
}

fn parse_input(input: &str) -> Vec<Valve> {
    let mut valves: Vec<Valve> = Vec::new();
    // Example line:
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    for line in input.lines() {
        let mut words = line.split_whitespace();

        let valve_id = valve_id_from_name(words.nth(1).unwrap());

        let flow_rate = words
            .nth(2)
            .unwrap()
            .strip_prefix("rate=")
            .unwrap()
            .strip_suffix(";")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let connections: Vec<u16> = words.skip(4).map(valve_id_from_name).collect();

        valves.push(Valve {
            connections: connections,
            flow_rate: flow_rate,
            id: valve_id,
        });
    }

    return valves;
}

fn compute_cost_map(valves: &Vec<Valve>, origin: u16) -> HashMap<u16, u16> {
    let mut cost_map: HashMap<u16, u16> = valves.iter().map(|v| (v.id, std::u16::MAX)).collect();

    _ = cost_map.insert(origin, 0);

    loop {
        let mut updated = false;

        for valve in valves.iter() {
            let cost = cost_map.get(&valve.id).unwrap();
            // Skip origin. Also allows safely subtracting 1 later on.
            if *cost == 0 {
                continue;
            }

            let min_neighbour_cost = valve
                .connections
                .iter()
                .filter_map(|id| cost_map.get(id))
                .min()
                .unwrap();

            if min_neighbour_cost < &(cost - 1) {
                cost_map.insert(valve.id, min_neighbour_cost + 1);
                updated = true;
            }
        }

        if updated == false {
            // Keep only the valves with a non 0 flow rate
            cost_map.retain(|k, _| {
                k == &0
                    || valves
                        .iter()
                        .find(|valve| &valve.id == k)
                        .unwrap()
                        .flow_rate
                        != 0
            });
            return cost_map;
        }
    }
}

fn compute_openable_valves(valves: &Vec<Valve>) -> Vec<ComputedValve> {
    let mut openable_valves: Vec<ComputedValve> = Vec::new();
    for valve in valves.iter() {
        if valve.flow_rate == 0 {
            continue;
        }

        let cost_map = compute_cost_map(&valves, valve.id);

        openable_valves.push(ComputedValve {
            id: valve.id,
            flow_rate: valve.flow_rate,
            cost_map: cost_map,
        });
    }
    return openable_valves;
}

#[derive(Clone)]
struct ComputedValve {
    id: u16,
    flow_rate: u64,
    cost_map: HashMap<u16, u16>,
}

const TIMEOUT: u16 = 30;

fn compute_flow(opened: &Vec<(u16, u16)>, openable: &Vec<ComputedValve>) -> u64 {
    opened
        .iter()
        .map(|(id, t)| {
            openable
                .iter()
                .find(|valve| valve.id == *id)
                .unwrap()
                .flow_rate as u64
                * (TIMEOUT as u64 - *t as u64)
        })
        .sum()
}

fn recursive_flow_computation(
    openable: &Vec<ComputedValve>,
    opened: &mut Vec<(u16, u16)>,
    position: u16,
    time: u16,
) -> u64 {
    let mut best = compute_flow(opened, openable);

    let origin = openable.iter().find(|valve| valve.id == position).unwrap();

    for valve in openable.iter() {
        if opened.iter().find(|(id, _)| id == &valve.id).is_some() {
            continue;
        }

        let next_time = time + origin.cost_map.get(&valve.id).unwrap() + 1;

        if next_time < TIMEOUT {
            opened.push((valve.id, next_time));
            best = max(
                best,
                recursive_flow_computation(openable, opened, valve.id, next_time),
            );
            _ = opened.pop();
        }
    }

    return best;
}

fn compute_best(
    openable: &Vec<ComputedValve>,
    origin_cost_map: &HashMap<u16, u16>,
    starting_cost: u16,
) -> u64 {
    let mut opened: Vec<(u16, u16)> = Vec::with_capacity(openable.len());
    openable
        .iter()
        .map(|valve| {
            let cost = origin_cost_map.get(&valve.id).unwrap() + starting_cost;
            opened.push((valve.id, cost));
            let result = recursive_flow_computation(&openable, &mut opened, valve.id, cost);
            _ = opened.pop();
            result
        })
        .max()
        .unwrap()
}

// Brute force
fn solve_part1(input: &str) -> u64 {
    let valves = parse_input(input);
    let openable = compute_openable_valves(&valves);
    // ID of "AA" is 0
    let origin_cost_map = compute_cost_map(&valves, 0);

    compute_best(&openable, &origin_cost_map, 1)
}

// Brute force 2 electric boogaloo
fn solve_part2(input: &str) -> u64 {
    let valves = parse_input(input);
    let openable = compute_openable_valves(&valves);
    let origin_cost_map = compute_cost_map(&valves, 0);
    let mask_length = openable.len();
    let mut best = 0u64;

    assert_eq!(mask_length <= 16, true);

    // Always faster to have a non-zero split on both sides
    for mask in 1u16..(2u16.pow(mask_length as u32) - 1) {
        let mut elephant_valves: Vec<ComputedValve> = Vec::new();
        let mut self_valves: Vec<ComputedValve> = Vec::new();
        for shift in 0..(mask_length as u16) {
            if mask & (1 << shift) != 0 {
                elephant_valves.push(openable[shift as usize].clone());
            } else {
                self_valves.push(openable[shift as usize].clone());
            }
        }

        let elephant_best = compute_best(&elephant_valves, &origin_cost_map, 5);
        let self_best = compute_best(&self_valves, &origin_cost_map, 5);

        best = max(best, elephant_best + self_best);
    }

    return best;
}

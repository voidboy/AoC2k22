use std::collections::HashMap;
use std::fs;

const SIMULATION_TIME:usize = 26;
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

/*
    Day 16 ~ Path finding......................but bruteforce

    we need to find the path which releases the highest value
    of pressure in 30 minutes.
    Moving cost 1 minute, opening a valve cost 1 minute, after
    being opened, a valve releases his flow rate every minute

    V = FLOW_RATE * (REMAINING_TIME - (MOVE + OPEN))

    The best path isn't necessarily the path which targets the
    highest flow_rate valves, in 30 minutes we can open at most
    15 valves, our input has 46 valves, 32 of them have 0 flow_rate
    so it "only" remains 14 effective valves :

    this makes a total of 14! = 87178291200 combinations.

    We could compute the average distance between each effective
    valves but we can just assume that it will be at least 2, thus,
    we should now look for a combinations of at most 7 valves.

    this lowers down the number of combinations :

        14 * 13 * 12 * 11 * 10 * 9 * 8 = 17297280 combinations.

    We are going to generate ALL combinations and pick the best one.

*/

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: usize,
    leads_to: Vec<&'a str>,
}

fn is_last_combination(
    combinaison: &Vec<usize>,
    effective_valves: &Vec<&Valve>,
    combination_size: usize,
) -> bool {
    for (index, val) in (effective_valves.len() - combination_size..effective_valves.len())
        .rev()
        .enumerate()
    {
        if combinaison[index] != val {
            return false;
        }
    }
    true
}

fn best_valve_to_valve_path(from: &str, to: &str, valves_indx: &HashMap<&str, Valve>) -> usize {
    let to_valve = valves_indx.get(to).unwrap();
    if to_valve.leads_to.contains(&from) {
        1
    } else {
        let from_valve = valves_indx.get(from).unwrap();
        let mut frontier = vec![];
        frontier.push(from);
        let mut cames_from: HashMap<&str, Option<&str>> = HashMap::new();
        cames_from.insert(from, None);
        loop {
            let current = frontier.pop().unwrap();
            if current == to {
                break ;
            }
            for neighbor in &valves_indx.get(current).unwrap().leads_to {
                if cames_from.get(neighbor).is_none() {
                    frontier.insert(0, neighbor);
                    cames_from.insert(neighbor, Some(current));
                }
            }
        }
        let mut found = cames_from.get(to);
        let mut count = 0;
        while found.unwrap().is_some() {
            found = cames_from.get(found.unwrap().unwrap());
            count += 1;
        }
        count
    }
}

fn simulate_path(path: &Vec<&Valve>, valves_indx: &HashMap<&str, Valve>) -> usize {
    // build
    let mut last_valve: Option<&Valve> = None;
    let mut cost: usize = usize::MIN;
    let mut triggers: Vec<usize> = vec![];
    for valve in path {
        if let Some(last) = last_valve {
            // cost of the move(s} + 1 for opening the valve
            cost += best_valve_to_valve_path(last.name, valve.name, valves_indx) + 1;
            triggers.push(cost);
        }
        last_valve = Some(valve);
    }
    // simulate
    let mut total_pressure: usize = usize::MIN;
    let mut pressure: usize = usize::MIN;
    let mut trigger_index: usize = usize::MIN;
    for tick in 0..=SIMULATION_TIME {
        total_pressure += pressure;
        if tick > 0 && trigger_index < triggers.len() && tick % triggers[trigger_index] == 0 {
            // trigger_index + 1, skip entrypoint
            pressure += path[trigger_index + 1].flow_rate;
            trigger_index += 1
        }
    }
    total_pressure
}

fn generate_best_combination(
    effective_valves: &Vec<&Valve>,
    valves_indx: &HashMap<&str, Valve>,
) -> usize {
    for combination_size in 2..=7 {
        let mut combination: Vec<usize> = vec![];
        let mut index: usize = usize::MIN;
        let mut max_pressure: usize = usize::MIN;
        for _ in 0..combination_size {
            combination.push(usize::MAX);
        }
        'outer: loop {
            'inner: for i in 0..effective_valves.len() {
                if !combination.contains(&i)
                    && (i > combination[index] || combination[index] == usize::MAX)
                {
                    combination[index] = i;
                    if index < combination_size - 1 {
                        index += 1;
                    } else {
                        //println!("Combination: {:?}", combination);
                        let mut path: Vec<&Valve> = vec![];
                        // ugly, adjust your starting point there
                        path.push(valves_indx.get("AA").unwrap());
                        for i in &combination {
                            path.push(effective_valves[*i]);
                        }
                        let this_pressure = simulate_path(&path, valves_indx);
                        if this_pressure > max_pressure {
                            max_pressure = this_pressure;
                        }
                        if is_last_combination(&combination, effective_valves, combination_size) {
                            break 'outer;
                        }
                    }
                    break 'inner;
                }
                // we reach the last value go back to n-1
                else if i == effective_valves.len() - 1 {
                    combination[index] = usize::MAX;
                    index -= 1;
                }
            }
        }
    }
    max_pressure
}

fn main() {
    let contents = fs::read_to_string("my_input").expect("Cannot read the file");

    let mut valves_indx: HashMap<&str, Valve> = HashMap::new();
    contents
        .lines()
        .map(|line| {
            let mut valve = line.split([' ', '=', ';', ',', '\n']);
            let name = valve.nth(1).unwrap();
            let flow_rate = valve.nth(3).unwrap().parse::<usize>().unwrap();
            let mut leads_to: Vec<&str> = vec![];
            for leading_valve in valve.skip(5) {
                if !leading_valve.is_empty() {
                    leads_to.push(leading_valve);
                }
            }
            Valve {
                name,
                flow_rate,
                leads_to,
            }
        })
        .for_each(|valve| {
            valves_indx.insert(valve.name, valve);
        });
    let mut effective_valves: Vec<&Valve> = vec![];
    for valve in valves_indx.values() {
        if valve.flow_rate > 0 {
            effective_valves.push(valve);
        }
    }
    let mut max_pressure: usize = usize::MIN;
    for combination_size in 2..=10 {
        let this_pressure =
            generate_best_combination(&effective_valves, &valves_indx, combination_size);
        if this_pressure > max_pressure {
            max_pressure = this_pressure;
            println!("New max_pressure found: {max_pressure}");
        }
    }
    println!("Part. I : {}", max_pressure);
}

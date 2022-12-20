use std::fs;

const BUILD_LENGTH: usize = 24;

#[derive(Debug, Copy, Clone)]
struct RobotFactoryState {
    ore: u8,
    ore_robots: u8,
    clay: u8,
    cla_robots: u8,
    obsidian: u8,
    obs_robots: u8,
    geode: u8,
    geo_robots: u8,
}

impl RobotFactoryState {
    fn new() -> Self {
        Self {
            ore: 0,
            ore_robots: 0,
            clay: 0,
            cla_robots: 0,
            obsidian: 0,
            obs_robots: 0,
            geode: 0,
            geo_robots: 0,
        }
    }

    fn genesis() -> Self {
        Self {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            cla_robots: 0,
            obsidian: 0,
            obs_robots: 0,
            geode: 0,
            geo_robots: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct RobotFactory {
    spec: BluePrint,
    states: [RobotFactoryState; BUILD_LENGTH],
}

impl RobotFactory {
    fn new(bp: BluePrint) -> Self {
        Self {
            spec: bp,
            states: [RobotFactoryState::new(); BUILD_LENGTH],
        }
    }

    fn run_from(self: &mut Self, from: usize, build: &[Action; BUILD_LENGTH]) -> u8 {
        for i in from..BUILD_LENGTH {
            self.states[i] = if i == 0 {
                RobotFactoryState::genesis()
            } else {
                self.states[i - 1]
            };
            let mut ore_build = 0;
            let mut cla_build = 0;
            let mut obs_build = 0;
            let mut geo_build = 0;

            match build[i] {
                Action::BuildOre => {
                    if self.states[i].ore >= self.spec.ore_robot_cost {
                        self.states[i].ore = self.states[i].ore - self.spec.ore_robot_cost;
                        ore_build += 1;
                    }
                }
                Action::BuildCla => {
                    if self.states[i].ore >= self.spec.cla_robot_cost {
                        self.states[i].ore = self.states[i].ore - self.spec.cla_robot_cost;
                        cla_build += 1;
                    }
                }
                Action::BuildObs => {
                    if self.states[i].ore >= self.spec.obs_robot_cost.0
                        && self.states[i].clay >= self.spec.obs_robot_cost.1
                    {
                        self.states[i].ore = self.states[i].ore - self.spec.obs_robot_cost.0;
                        self.states[i].clay = self.states[i].clay - self.spec.obs_robot_cost.1;
                        obs_build += 1;
                    }
                }
                Action::BuildGeo => {
                    if self.states[i].ore >= self.spec.geo_robot_cost.0
                        && self.states[i].obsidian >= self.spec.geo_robot_cost.1
                    {
                        self.states[i].ore = self.states[i].ore - self.spec.geo_robot_cost.0;
                        self.states[i].obsidian =
                            self.states[i].obsidian - self.spec.geo_robot_cost.1;
                        geo_build += 1;
                    }
                }
                Action::Nothing => {}
            }
            // collect
            self.states[i].ore += self.states[i].ore_robots;
            self.states[i].clay += self.states[i].cla_robots;
            self.states[i].obsidian += self.states[i].obs_robots;
            self.states[i].geode += self.states[i].geo_robots;
            // end_build
            self.states[i].ore_robots += ore_build;
            self.states[i].cla_robots += cla_build;
            self.states[i].obs_robots += obs_build;
            self.states[i].geo_robots += geo_build;
        }
        self.states[BUILD_LENGTH - 1].geode
    }
}

#[derive(Debug, Copy, Clone)]
struct BluePrint {
    id: u8,
    ore_robot_cost: u8,
    cla_robot_cost: u8,
    obs_robot_cost: (u8, u8),
    geo_robot_cost: (u8, u8),
}

#[derive(Debug, Copy, PartialEq, Clone)]
enum Action {
    Nothing,
    BuildOre,
    BuildCla,
    BuildObs,
    BuildGeo,
}

impl Action {
    fn next(self: &Self) -> Self {
        match self {
            Self::Nothing => Self::BuildOre,
            Self::BuildOre => Self::BuildCla,
            Self::BuildCla => Self::BuildObs,
            Self::BuildObs => Self::BuildGeo,
            Self::BuildGeo => Self::Nothing,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the file");

    let blueprints: Vec<BluePrint> = contents
        .lines()
        .map(|line| {
            let mut blueprint = line.split(&[' ', ':']);
            BluePrint {
                id: blueprint.nth(1).unwrap().parse::<u8>().unwrap(),
                ore_robot_cost: blueprint.nth(5).unwrap().parse::<u8>().unwrap(),
                cla_robot_cost: blueprint.nth(5).unwrap().parse::<u8>().unwrap(),
                obs_robot_cost: (
                    blueprint.nth(5).unwrap().parse::<u8>().unwrap(),
                    blueprint.nth(2).unwrap().parse::<u8>().unwrap(),
                ),
                geo_robot_cost: (
                    blueprint.nth(5).unwrap().parse::<u8>().unwrap(),
                    blueprint.nth(2).unwrap().parse::<u8>().unwrap(),
                ),
            }
        })
        .collect::<Vec<BluePrint>>();

    /* 
        Atm, there are still too much combinations to test, we need to find a 
        way to reduce this number. We added memoization but it doesn't seems 
        to help a lot. 
    */
    let mut max_geode: u8 = u8::MIN;
    let p1: usize = blueprints.into_iter().map(|bp| {
        println!("Testing blueprint {}..", bp.id);
        let mut build: [Action; BUILD_LENGTH] = [Action::Nothing; BUILD_LENGTH];
        println!("{:?}", build);
        let mut robot_factory = RobotFactory::new(bp);
        let mut memoization: usize = 0;
        let minimum_start = std::cmp::min(bp.ore_robot_cost, bp.cla_robot_cost);
        'outer: loop {
            let this_geode = robot_factory.run_from(memoization, &build);
            if this_geode > max_geode {
                max_geode = this_geode;
                println!("best[{}]: {max_geode} with {:?}", bp.id, build);
            }
            'inner: for i in (minimum_start as usize..BUILD_LENGTH - 1).rev() {
                if build[i] != Action::BuildGeo {
                    build[i] = build[i].next();
                    if i == minimum_start as usize && build[i] == Action::BuildObs {
                        break 'outer;
                    }
                    memoization = i;
                    break 'inner;
                } else {
                    build[i] = Action::Nothing;
                }
            }
        }
        bp.id as usize * max_geode as usize
    }).sum();
    println!("Part. I : {p1}");
}

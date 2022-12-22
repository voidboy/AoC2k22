use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum MonkeyType {
    Yell,
    Math,
}

#[derive(Copy, Clone, Debug)]
struct MathJob<'a> {
    left: &'a str,
    operator: &'a str,
    right: &'a str,
}

#[derive(Copy, Clone)]
union Job<'a> {
    number: f64,
    math: MathJob<'a>,
}

#[derive(Copy, Clone)]
struct Monkey<'a> {
    name: &'a str,
    monkey_type: MonkeyType,
    job: Job<'a>,
}

fn get_value(this: &Monkey, others: &HashMap<&str, Monkey>) -> f64 {
    match this.monkey_type {
        MonkeyType::Yell => unsafe { this.job.number },
        MonkeyType::Math => {
            let mjob = unsafe { this.job.math };
            match mjob.operator {
                "+" => {
                        get_value(&others[mjob.left], others)
                        + get_value(&others[mjob.right], others)
                }
                "-" => {
                        get_value(&others[mjob.left], others)
                        - get_value(&others[mjob.right], others)
                }
                "*" => {
                        get_value(&others[mjob.left], others)
                        * get_value(&others[mjob.right], others)
                }
                "/" => {
                        get_value(&others[mjob.left], others) 
                        / get_value(&others[mjob.right], others)
                }
                _ => panic!("Unsupported operator"),
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("my_input").expect("Cannot read the file");
    let mut monkeys: HashMap<&str, Monkey> = HashMap::new();
    contents.lines().for_each(|line| {
        let mut monkey = line.split(':');
        let monkey_type: MonkeyType;
        let job: Job;
        let name = monkey.next().unwrap().trim();
        let mjob = monkey.next().unwrap().trim();
        match mjob.parse::<f64>() {
            Err(_) => {
                monkey_type = MonkeyType::Math;
                let mut math_job = mjob.split_whitespace();
                job = {
                    Job {
                        math: MathJob {
                            left: math_job.next().unwrap(),
                            operator: math_job.next().unwrap(),
                            right: math_job.next().unwrap(),
                        },
                    }
                }
            }
            Ok(n) => {
                monkey_type = MonkeyType::Yell;
                job = Job { number: n };
            }
        }
        // we store the name 2 times, as key and inside the value ?
        monkeys.insert(
            name,
            Monkey {
                name,
                monkey_type,
                job,
            },
        );
    });
    println!("Part. I: {}", get_value(&monkeys["root"], &monkeys));
}

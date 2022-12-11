use std::fs;

/*

Monkey 0:
  Starting items: 98, 89, 52
  Operation: new = old * 2
  Test: divisible by 5
    If true: throw to monkey 6
    If false: throw to monkey 1

*/

#[derive(Debug)]
struct Monkey<F>
where
    F: Fn(f64, f64) -> f64,
{
    items: Vec<f64>,
    operation: F,
    operation_val: f64,
    divisible_by: f64,
    if_true: f64,
    if_false: f64,
    inspected_items: usize,
}

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the file");

    let add = |a: f64, b: f64| -> f64 { a + b };
    let mul = |a: f64, b: f64| -> f64 { a * b };
    let mut monkeys = contents
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.split("\n").skip(1);
            // Starting items: 98, 89, 52
            let items = &lines.next().unwrap()[18..]
                .split(",")
                .flat_map(|n| n.trim().parse::<f64>())
                .collect::<Vec<f64>>();
            // Operation: new = old * 2
            let operation_line = &mut lines.next().unwrap()[23..].split_whitespace();
            let operation = match operation_line.next().unwrap() {
                "*" => mul,
                "+" => add,
                _ => panic!("unreconized operation"),
            };
            let operation_val = operation_line.next().unwrap().parse::<f64>().unwrap_or(0.0);
            // Test: divisible by 5
            let divisible_by = &lines.next().unwrap()[21..].parse::<f64>().unwrap();
            // If true: throw to monkey 6
            let if_true = &lines.next().unwrap()[29..].parse::<f64>().unwrap();
            // If false: throw to monkey 1
            let if_false = &lines.next().unwrap()[30..].parse::<f64>().unwrap();
            Monkey {
                items: items.to_vec(),
                operation,
                operation_val,
                divisible_by: *divisible_by,
                if_true: *if_true,
                if_false: *if_false,
                inspected_items: 0,
            }
        })
        .collect::<Vec<Monkey<_>>>();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let worry_level = if monkeys[i].operation_val > 0.0 {
                    (monkeys[i].operation)(monkeys[i].items[j], monkeys[i].operation_val)
                } else {
                    (monkeys[i].operation)(monkeys[i].items[j], monkeys[i].items[j])
                };
                let new_worry_level = (worry_level / 3.0).floor();
                let wtf1 = monkeys[i].if_true as usize;
                let wtf2 = monkeys[i].if_false as usize;
                let target_monkey = if new_worry_level % monkeys[i].divisible_by == 0.0 {
                    &mut monkeys[wtf1]
                } else {
                    &mut monkeys[wtf2]
                };
                target_monkey.items.push(new_worry_level);
            }
            monkeys[i].inspected_items += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
    monkeys.sort_by(|jimmy, duck| duck.inspected_items.cmp(&jimmy.inspected_items));
    println!(
        "Part. I: {}",
        monkeys[0].inspected_items * monkeys[1].inspected_items
    );
}

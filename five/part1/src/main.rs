use std::{collections::{HashMap, HashSet}, sync::LazyLock};

static CONDITIONS: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../conditions").unwrap());
static DATA: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../data").unwrap());

#[derive(Debug)]
struct Condition {
    before: HashSet<usize>,
    after: HashSet<usize>,
}

fn convert_line(line: &str, split_char: char) -> Vec<usize> {
    line.split(split_char)
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn line_is_correct(line: &[usize], conditions: &mut HashMap<usize, Condition>) -> bool {
    for (num_index, num) in line.iter().enumerate() {
        let next_nums = &line[num_index + 1..];
        let num_condition_sum = &conditions.get(num).unwrap().after;
        for next_num in next_nums {
            if !num_condition_sum.contains(next_num) {
                return false;
            }
        }
    }

    true
}

fn main() {
    let mut conditions: HashMap<usize, Condition> = HashMap::new();

    for line in CONDITIONS.lines() {
        let pair = convert_line(line, '|');

        let lhs = pair[0];
        let rhs = pair[1];

        conditions
            .entry(lhs)
            .and_modify(|num| {
                num.after.insert(rhs);
            })
            .or_insert(Condition {
                before: HashSet::new(),
                after: HashSet::from([rhs]),
            });

        conditions
            .entry(rhs)
            .and_modify(|num| {
                num.before.insert(lhs);
            })
            .or_insert(Condition {
                before: HashSet::from([lhs]),
                after: HashSet::new(),
            });
    }

    let mut sum = 0;

    for line in DATA.lines() {
        let line_parsed = convert_line(line, ',');

        if line_is_correct(&line_parsed, &mut conditions) {
            sum += line_parsed[line_parsed.len() / 2]
        }
    }

    println!("{}", sum);
}

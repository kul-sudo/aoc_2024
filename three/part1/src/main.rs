use regex::Regex;
use std::sync::LazyLock;

static STRING: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../string").unwrap());

fn main() {
    let regex = Regex::new(r"mul\((?P<lhs>\d{1,3}),(?P<rhs>\d{1,3})\)").unwrap();

    let mut sum = 0;

    for line in STRING.lines() {
        for capture in regex.captures_iter(line) {
            let lhs = capture["lhs"].parse::<usize>().unwrap();
            let rhs = capture["rhs"].parse::<usize>().unwrap();

            sum += lhs * rhs;
        }
    }

    println!("{}", sum);
}

use regex::Regex;
use std::sync::LazyLock;

static STRING: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../string").unwrap());

#[derive(PartialEq)]
enum Mode {
    Do,
    DoNot,
}

fn main() {
    let regex = Regex::new(
        r"(?P<mul>mul\((?P<lhs>\d{1,3}),(?P<rhs>\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))",
    )
    .unwrap();

    let mut sum = 0;

    let mut mode = Mode::Do;

    for line in STRING.lines() {
        for capture in regex.captures_iter(line) {
            if capture.name("do").is_some() {
                mode = Mode::Do;
            } else if capture.name("dont").is_some() {
                mode = Mode::DoNot
            }

            if mode == Mode::Do {
                if capture.name("mul").is_some() {
                    let lhs = capture["lhs"].parse::<usize>().unwrap();
                    let rhs = capture["rhs"].parse::<usize>().unwrap();
                    sum += lhs * rhs;
                }
            }
        }
    }

    println!("{}", sum);
}

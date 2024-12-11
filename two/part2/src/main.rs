use std::sync::LazyLock;

static STRING: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../string").unwrap());

#[derive(PartialEq)]
enum Order {
    Increasing,
    Decreasing,
}

fn is_line_safe_base(numbers: &[usize]) -> bool {
    let mut order = None;

    for (number_index, number) in numbers.iter().enumerate().take(numbers.len() - 1) {
        if *number > numbers[number_index + 1] {
            if order.is_some_and(|order| order == Order::Increasing) {
                return false;
            }
            order = Some(Order::Decreasing);
        }

        if *number < numbers[number_index + 1] {
            if order.is_some_and(|order| order == Order::Decreasing) {
                return false;
            }
            order = Some(Order::Increasing);
        }

        if !(1..=3).contains(&number.abs_diff(numbers[number_index + 1])) {
            return false;
        }
    }

    true
}

fn is_line_safe(line: &str) -> bool {
    let numbers = line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    if is_line_safe_base(&numbers) {
        return true;
    } else {
        for number_index in 0..numbers.len() {
            let numbers_modified = numbers
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != number_index)
                .map(|x| *x.1)
                .collect::<Vec<_>>();

            if is_line_safe_base(&numbers_modified) {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut n_safe = 0;

    for line in STRING.lines() {
        n_safe += is_line_safe(line) as usize;
    }

    println!("{}", n_safe);
}

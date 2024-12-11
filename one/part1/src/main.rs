use std::sync::LazyLock;

static STRING: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../string").unwrap());

fn main() {
    let mut left_column = Vec::new();
    let mut right_column = Vec::new();

    for line in STRING.lines() {
        let numbers = line.split("   ").collect::<Vec<_>>();
        left_column.push(numbers[0].parse::<usize>().unwrap());
        right_column.push(numbers[1].parse::<usize>().unwrap());
    }

    left_column.sort();
    right_column.sort();

    let mut sum = 0;
    for i in 0..left_column.len() {
        sum += (left_column[i]).abs_diff(right_column[i]);
    }

    println!("{}", sum);
}

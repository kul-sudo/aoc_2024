use std::sync::LazyLock;

const STRING: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

static MAX_X: LazyLock<usize> = LazyLock::new(|| STRING.chars().count() / STRING.lines().count());
static MAX_Y: LazyLock<usize> = LazyLock::new(|| STRING.lines().count());
static PHRASE: &str = "XMAS";

fn index(column: usize, row: usize) -> char {
    STRING.chars().nth(row * *MAX_X + column).unwrap()
}

fn handle_storage(char: char, n: &mut usize, storage: &mut String) {
    if STRING.chars().nth(storage.len()) == Some(char) {
        storage.push(char)
    } else {
        storage.clear();
    }

    if storage.len() == PHRASE.len() {
        *n += 1;
        storage.clear();
    }
}

fn main() {
    // Vertical
    let mut vertical_n = 0;

    let mut regular_vertical_storage = String::new();
    for j in 0..*MAX_X {
        for i in 0..*MAX_Y {
            let char = index(j, i);
            handle_storage(char, &mut vertical_n, &mut regular_vertical_storage);
        }
    }

    let mut reversed_vertical_storage = String::new();
    for j in 0..*MAX_X {
        for i in (0..*MAX_Y).rev() {
            let char = index(j, i);
            handle_storage(char, &mut vertical_n, &mut reversed_vertical_storage);
        }
    }

    // Horizontal
    let mut horizontal_n = 0;

    let mut regular_horizontal_storage = String::new();
    for i in 0..*MAX_Y {
        for j in 0..*MAX_X {
            let char = index(j, i);
            handle_storage(char, &mut horizontal_n, &mut regular_horizontal_storage);
        }
    }

    let mut reversed_horizontal_storage = String::new();
    for i in 0..*MAX_Y {
        for j in (0..*MAX_X).rev() {
            let char = index(j, i);
            handle_storage(char, &mut horizontal_n, &mut reversed_horizontal_storage);
        }
    }

    let sum = vertical_n + horizontal_n;

    println!("{}", sum);
}

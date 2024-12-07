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
MXMXAXMASX
";

static MAX_X: LazyLock<usize> = LazyLock::new(|| STRING.chars().count() / STRING.lines().count());
static MAX_Y: LazyLock<usize> = LazyLock::new(|| STRING.lines().count());
static PHRASE: &str = "XMAS";

fn index(column: usize, row: usize) -> char {
    STRING.chars().nth(row * *MAX_X + column).unwrap()
}

fn check_south(i: usize, j: usize) -> bool {
    for p in 0..PHRASE.len() {
        if index(j, i + p) != PHRASE.chars().nth(p).unwrap() {
            return false;
        }
    }

    true
}

fn check_north(i: usize, j: usize) -> bool {
    for p in 0..PHRASE.len() {
        if index(j, i - p) != PHRASE.chars().nth(p).unwrap() {
            return false;
        }
    }

    true
}

fn check_west(i: usize, j: usize) -> bool {
    for p in 0..PHRASE.len() {
        if index(j - p, i) != PHRASE.chars().nth(p).unwrap() {
            return false;
        }
    }

    true
}

fn check_east(i: usize, j: usize) -> bool {
    for p in 0..PHRASE.len() {
        if index(j + p, i) != PHRASE.chars().nth(p).unwrap() {
            return false;
        }
    }

    true
}

fn check_direction(mut i: usize, mut j: usize, direction: (isize, isize)) -> bool {
    let initial_i = i;
    let initial_j = j;

    for p in 0..PHRASE.len() {
        let i_new = initial_i as isize + p as isize * direction.1;
        let j_new = initial_j as isize + p as isize * direction.0;

        if !(0..*MAX_X as isize).contains(&j_new) || !(0..*MAX_Y as isize).contains(&i_new) {
            return false;
        }

        i = i_new as usize;
        j = j_new as usize;

        if index(j, i) != PHRASE.chars().nth(p).unwrap() {
            return false;
        }
    }

    true
}

fn main() {
    let mut n = 0;

    for i in 0..*MAX_Y {
        for j in 0..*MAX_X {
            if *MAX_Y >= PHRASE.len() {
                // South
                if (0..=*MAX_Y - PHRASE.len()).contains(&i) && check_south(i, j) {
                    n += 1;
                }

                // North
                if (PHRASE.len()..*MAX_X).contains(&i) && check_north(i, j) {
                    n += 1;
                }
            }

            // West
            if (PHRASE.len()..*MAX_X).contains(&j) && check_west(i, j) {
                n += 1;
            }

            // East
            if (0..=*MAX_X - PHRASE.len()).contains(&j) && check_east(i, j) {
                n += 1;
            }

            // Around
            for direction in [(1, 1)] {
                if check_direction(i, j, direction) {
                    n += 1
                }
            }
        }
    }

    println!("{}", n * 2);
}

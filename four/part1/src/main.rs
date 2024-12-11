use std::sync::LazyLock;

static STRING: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../string").unwrap());

static MAX_X: LazyLock<usize> = LazyLock::new(|| STRING.chars().count() / STRING.lines().count());
static MAX_Y: LazyLock<usize> = LazyLock::new(|| STRING.lines().count());
static PHRASE: &str = "XMAS";

fn index(column: usize, row: usize) -> char {
    STRING.chars().nth(row * *MAX_X + column).unwrap()
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

        if PHRASE.chars().nth(p) != Some(index(j, i)) {
            return false;
        }
    }

    true
}

fn main() {
    let mut n = 0;

    for i in 0..*MAX_Y {
        for j in 0..*MAX_X {
            for direction in [
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
            ] {
                if check_direction(i, j, direction) {
                    n += 1
                }
            }
        }
    }

    println!("{}", n);
}

use std::sync::LazyLock;

static STRING: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string("../string").unwrap());

static MAX_X: LazyLock<usize> = LazyLock::new(|| STRING.chars().count() / STRING.lines().count());
static MAX_Y: LazyLock<usize> = LazyLock::new(|| STRING.lines().count());

fn index(column: usize, row: usize) -> Option<char> {
    STRING.chars().nth(row * *MAX_X + column)
}

fn main() {
    let mut n = 0;

    for i in 0..*MAX_Y {
        for j in 0..*MAX_X {
            let char = index(j, i);

            if char == Some('A') {
                let mut lhs = [None; 2];
                let mut rhs = [None; 2];

                lhs[0] = index(j - 1, i - 1);
                lhs[1] = index(j - 1, i + 1);

                rhs[0] = index(j + 1, i - 1);
                rhs[1] = index(j + 1, i + 1);

                if (lhs == [Some('M'); 2] && rhs == [Some('S'); 2])
                    || (lhs == [Some('S'); 2] && rhs == [Some('M'); 2]
                        || (lhs == [Some('M'), Some('S')] && rhs == [Some('M'), Some('S')])
                        || (lhs == [Some('S'), Some('M')] && rhs == [Some('S'), Some('M')]))
                {
                    n += 1;
                }
            }
        }
    }

    println!("{}", n);
}

use once_cell::sync::Lazy;
use regex::Regex;

static PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"inp w\nmul x 0\nadd x z\nmod x 26\ndiv z (1|26)\nadd x (-?\d+)\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y (-?\d+)\nmul y x\nadd z y\n").unwrap()
});

fn parse_params(input: &str) -> Vec<(isize, isize, isize)> {
    let parse_captured = |cap: &regex::Captures, idx: usize| {
        cap.get(idx).unwrap().as_str().parse::<isize>().unwrap()
    };
    let mut params = Vec::new();
    for cap in PATTERN.captures_iter(input) {
        params.push((
            parse_captured(&cap, 1),
            parse_captured(&cap, 2),
            parse_captured(&cap, 3),
        ));
    }
    params
}

fn solve(params: &[(isize, isize, isize)], highest: bool) -> isize {
    let mut digits: [Option<isize>; 14] = Default::default();
    let mut stack = Vec::new();
    for (idx, (a, b, c)) in params.iter().enumerate() {
        if *a == 1 {
            stack.push((idx, c))
        } else {
            let (other_idx, base) = stack.pop().unwrap();
            let diff = base + b;
            digits[other_idx] = Some(if highest {
                if diff < 0 {
                    9
                } else {
                    9 - diff
                }
            } else if diff < 0 {
                1 - diff
            } else {
                1
            });
            digits[idx] = Some(digits[other_idx].unwrap() + diff);
        }
    }
    digits
        .into_iter()
        .map(Option::unwrap)
        .fold(0, |acc, n| acc * 10 + n)
}

fn main() {
    let params = parse_params(include_str!("../../inputs/day24.txt"));
    println!("Part 1: {}", solve(&params, true));
    println!("Part 2: {}", solve(&params, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_params() {
        let params = parse_params(include_str!("../../inputs/day24.txt"));
        assert_eq!(
            params,
            [
                (1, 10, 10,),
                (1, 13, 5,),
                (1, 15, 12,),
                (26, -12, 12,),
                (1, 14, 6,),
                (26, -2, 4,),
                (1, 13, 15,),
                (26, -12, 3,),
                (1, 15, 7,),
                (1, 11, 11,),
                (26, -3, 2,),
                (26, -13, 12,),
                (26, -12, 4,),
                (26, -13, 11,),
            ]
        );
    }
}

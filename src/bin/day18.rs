use std::fmt::Write;

use logos::Logos;

#[derive(Logos, Debug, Eq, PartialEq)]
enum Token {
    #[error]
    Error,

    #[token("[")]
    Open,

    #[token("]")]
    Close,

    #[token(",", logos::skip)]
    Comma,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Num(usize),
}

#[derive(Clone, Eq, PartialEq)]
enum Pair {
    Num(usize),
    Cons(Box<Pair>, Box<Pair>),
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => f.write_fmt(format_args!("{}", n)),
            Self::Cons(left, right) => {
                f.write_char('[')?;
                left.fmt(f)?;
                f.write_char(',')?;
                right.fmt(f)?;
                f.write_char(']')
            }
        }
    }
}

impl Pair {
    fn parse(s: &str) -> Option<Self> {
        let mut incomplete_pairs: Vec<Option<Pair>> = Vec::new();
        for token in Token::lexer(s) {
            match token {
                Token::Error => return None,
                Token::Open => incomplete_pairs.push(None),
                Token::Close => {}
                Token::Comma => {}
                Token::Num(n) => {
                    let mut current_pair = Pair::Num(n);
                    loop {
                        if let Some(pair_opt) = incomplete_pairs.pop() {
                            if let Some(pair) = pair_opt {
                                current_pair = pair.add(current_pair);
                            } else {
                                incomplete_pairs.push(Some(current_pair));
                                break;
                            }
                        } else {
                            return Some(current_pair);
                        }
                    }
                }
            }
        }
        None
    }

    fn parse_multi(s: &str) -> Option<Vec<Self>> {
        s.lines().map(Pair::parse).collect()
    }

    fn add(self, rhs: Self) -> Self {
        Self::Cons(Box::new(self), Box::new(rhs))
    }

    fn get_num(&self) -> Option<usize> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    fn explode(&mut self) -> Option<(usize, usize)> {
        self.explode_at_depth(4)
    }

    fn propagate_left_explode(&mut self, val: usize) {
        match self {
            Self::Num(n) => *n += val,
            Self::Cons(_, right) => right.propagate_left_explode(val),
        }
    }

    fn propagate_right_explode(&mut self, val: usize) {
        match self {
            Self::Num(n) => *n += val,
            Self::Cons(left, _) => left.propagate_right_explode(val),
        }
    }

    fn explode_at_depth(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Self::Num(_) => None,
            Self::Cons(left, right) => {
                if depth == 0 {
                    Some((left.get_num().unwrap(), right.get_num().unwrap()))
                } else if let Some((l, r)) = left.explode_at_depth(depth - 1) {
                    right.propagate_right_explode(r);
                    if depth == 1 {
                        *left = Box::new(Self::Num(0));
                    }
                    Some((l, 0))
                } else if let Some((l, r)) = right.explode_at_depth(depth - 1) {
                    left.propagate_left_explode(l);
                    if depth == 1 {
                        *right = Box::new(Self::Num(0));
                    }
                    Some((0, r))
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Self::Num(n) => {
                if *n >= 10 {
                    Some(())
                } else {
                    None
                }
            }
            Self::Cons(left, right) => {
                if left.split().is_some() {
                    if let Some(n) = left.get_num() {
                        *left = Box::new(Self::Num(n / 2).add(Self::Num(n / 2 + n % 2)));
                    }
                    Some(())
                } else if right.split().is_some() {
                    if let Some(n) = right.get_num() {
                        *right = Box::new(Self::Num(n / 2).add(Self::Num(n / 2 + n % 2)));
                    }
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            while self.explode().is_some() {}
            if self.split().is_none() {
                return;
            }
        }
    }

    fn sum(pairs: &[Self]) -> Self {
        let (first, rest) = pairs.split_first().unwrap();
        let mut result = first.clone();
        for pair in rest {
            result = result.add(pair.clone());
            result.reduce();
        }
        result
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Num(n) => *n,
            Self::Cons(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn find_largest_magnitude_of_sum_of_two(pairs: &[Self]) -> usize {
        let mut max_magnitude = 0;
        for i in 0..pairs.len() {
            for j in 0..pairs.len() {
                if i != j {
                    let mut sum = pairs[i].clone().add(pairs[j].clone());
                    sum.reduce();
                    let magnitude = sum.magnitude();
                    if max_magnitude < magnitude {
                        max_magnitude = magnitude;
                    }
                }
            }
        }
        max_magnitude
    }
}

fn main() {
    let nums = Pair::parse_multi(include_str!("../../inputs/day18.txt")).unwrap();
    println!("Part 1: {}", Pair::sum(&nums).magnitude());
    println!(
        "Part 2: {}",
        Pair::find_largest_magnitude_of_sum_of_two(&nums)
    );
}

#[cfg(test)]
mod tests {
    use super::Pair::*;
    use super::*;

    #[test]
    fn test_pair_parse() {
        assert_eq!(Pair::parse(""), None);
        assert_eq!(Pair::parse("[1,"), None);

        // not very strict here
        assert_eq!(
            Pair::parse("[1,2,3"),
            Some(Cons(Box::new(Num(1)), Box::new(Num(2))))
        );

        assert_eq!(
            Pair::parse("[1,2]"),
            Some(Cons(Box::new(Num(1)), Box::new(Num(2))))
        );
        assert_eq!(
            Pair::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"),
            Some(Cons(
                Box::new(Cons(
                    Box::new(Cons(
                        Box::new(Cons(Box::new(Num(1)), Box::new(Num(3)),)),
                        Box::new(Cons(Box::new(Num(5)), Box::new(Num(3)),)),
                    )),
                    Box::new(Cons(
                        Box::new(Cons(Box::new(Num(1)), Box::new(Num(3)),)),
                        Box::new(Cons(Box::new(Num(8)), Box::new(Num(7)),)),
                    )),
                )),
                Box::new(Cons(
                    Box::new(Cons(
                        Box::new(Cons(Box::new(Num(4)), Box::new(Num(9)),)),
                        Box::new(Cons(Box::new(Num(6)), Box::new(Num(9)),)),
                    )),
                    Box::new(Cons(
                        Box::new(Cons(Box::new(Num(8)), Box::new(Num(2)),)),
                        Box::new(Cons(Box::new(Num(7)), Box::new(Num(3)),)),
                    )),
                )),
            ))
        );
    }

    fn should<R, F: FnOnce(&mut Pair) -> Option<R>>(input: &str, transform: F, output: &str) {
        let mut num = Pair::parse(input).unwrap();
        assert!(transform(&mut num).is_some());
        assert_eq!(num, Pair::parse(output).unwrap());
    }

    #[test]
    fn test_explode() {
        should("[[[[[9,8],1],2],3],4]", Pair::explode, "[[[[0,9],2],3],4]");
        should("[7,[6,[5,[4,[3,2]]]]]", Pair::explode, "[7,[6,[5,[7,0]]]]");
        should("[[6,[5,[4,[3,2]]]],1]", Pair::explode, "[[6,[5,[7,0]]],3]");
        should(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            Pair::explode,
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        should(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            Pair::explode,
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        assert!(Pair::parse("[[[[[9,8],1],2],3],4]")
            .unwrap()
            .split()
            .is_none());
        should(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            Pair::split,
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        should(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            Pair::split,
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    fn reduce(pair: &mut Pair) -> Option<()> {
        pair.reduce();
        Some(())
    }

    #[test]
    fn test_reduce_sample() {
        should(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            reduce,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    fn list_should_sum_to(input: &str, output: &str) {
        let nums = Pair::parse_multi(input).unwrap();
        let result = Pair::sum(&nums);
        assert_eq!(result, Pair::parse(output).unwrap());
    }

    #[test]
    fn test_sum() {
        list_should_sum_to(
            "[1,1]\n[2,2]\n[3,3]\n[4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        list_should_sum_to(
            "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        list_should_sum_to(
            "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
    }

    #[test]
    fn test_complex_reduce() {
        let mut num = Pair::parse(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        )
        .unwrap();
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse(
                "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
            )
            .unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")
                .unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[15,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")
                .unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[0,[11,3]],[[6,3],[8,8]]]]]")
                .unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,0],[[9,3],[8,8]]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,9],[0,[11,8]]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,0],[[7,8],5]]],[10,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[0,13]]],[10,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[0,13]]],[10,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[0,[6,7]]]],[10,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[17,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[17,[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,9],[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,9],[[11,9],[11,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,9],[[[5,6],9],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,14],[[0,15],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,14],[[0,15],[11,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[0,15],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[0,15],[11,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[0,[7,8]],[11,0]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,0],[19,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,0],[19,0]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,0],[[9,10],0]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[0,10]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[0,10]]]]").unwrap()
        );
        assert!(num.split().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[0,[5,5]]]]]").unwrap()
        );
        assert!(num.explode().is_some());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap()
        );
        assert!(num.explode().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap()
        );
        assert!(num.split().is_none());
        assert_eq!(
            num,
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap()
        );
    }

    #[test]
    fn test_complex_reduce_but_with_reduce() {
        should(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            reduce,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
    }

    #[test]
    fn test_sum_longer() {
        list_should_sum_to(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
        list_should_sum_to(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
        );
        list_should_sum_to(
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
        );
        list_should_sum_to(
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]\n[7,[5,[[3,8],[1,4]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
        );
        list_should_sum_to(
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]\n[[2,[2,2]],[8,[8,1]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
        );
        list_should_sum_to(
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]\n[2,9]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
        );
        list_should_sum_to(
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]\n[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
        );
        list_should_sum_to(
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]\n[[[5,[7,4]],7],1]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
        );
        list_should_sum_to(
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]\n[[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );

        list_should_sum_to(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n[7,[5,[[3,8],[1,4]]]]\n[[2,[2,2]],[8,[8,1]]]\n[2,9]\n[1,[[[9,3],9],[[9,0],[0,7]]]]\n[[[5,[7,4]],7],1]\n[[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    fn test_part1_last_sample() {
        let nums = Pair::parse_multi("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]").unwrap();
        let num = Pair::sum(&nums);
        assert_eq!(
            num,
            Pair::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").unwrap()
        );
        assert_eq!(num.magnitude(), 4140);
    }

    #[test]
    fn test_part2_sample() {
        let nums = Pair::parse_multi("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]").unwrap();
        assert_eq!(Pair::find_largest_magnitude_of_sum_of_two(&nums), 3993);
    }
}

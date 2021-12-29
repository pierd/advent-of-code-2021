#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    East,
    South,
}

fn parse(input: &str) -> Vec<Vec<Option<Direction>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '>' => Some(Direction::East),
                    'v' => Some(Direction::South),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn step(map: &mut [Vec<Option<Direction>>]) -> usize {
    let mut moved = 0;
    let rows = map.len();
    let cols = map[0].len();

    let mut to_move = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, d) in row.iter().enumerate() {
            if *d == Some(Direction::East) && row[(col_idx + 1) % cols] == None {
                to_move.push((row_idx, col_idx));
            }
        }
    }
    moved += to_move.len();
    for (row, col) in to_move.into_iter() {
        map[row][col] = None;
        map[row][(col + 1) % cols] = Some(Direction::East);
    }

    let mut to_move = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, d) in row.iter().enumerate() {
            if *d == Some(Direction::South) && map[(row_idx + 1) % rows][col_idx] == None {
                to_move.push((row_idx, col_idx))
            }
        }
    }
    moved += to_move.len();
    for (row, col) in to_move.into_iter() {
        map[row][col] = None;
        map[(row + 1) % rows][col] = Some(Direction::South);
    }
    moved
}

fn count_steps(map: &[Vec<Option<Direction>>]) -> usize {
    let mut map = map.to_vec();
    let mut steps = 0;
    while step(&mut map) != 0 {
        steps += 1;
    }
    steps + 1
}

fn main() {
    let starting_map = parse(include_str!("../../inputs/day25.txt"));
    println!("Part 1: {}", count_steps(&starting_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut starting = parse("..........\n.>v....v..\n.......>..\n..........");
        let expected = parse("..........\n.>........\n..v....v>.\n..........");
        step(&mut starting);
        assert_eq!(starting, expected);
    }

    #[test]
    fn test_step_sample() {
        let mut map = parse(include_str!("../../inputs/day25-sample.txt"));
        while step(&mut map) > 0 {}
        assert_eq!(map, parse("..>>v>vv..\n..v.>>vv..\n..>>v>>vv.\n..>>>>>vv.\nv......>vv\nv>v....>>v\nvvv.....>>\n>vv......>\n.>v.vv.v.."));
    }

    #[test]
    fn test_count_steps() {
        let map = parse(include_str!("../../inputs/day25-sample.txt"));
        assert_eq!(count_steps(&map), 58);
    }
}

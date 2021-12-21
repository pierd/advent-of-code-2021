use std::collections::HashSet;

use once_cell::sync::Lazy;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Vector([isize; 4]);

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.0[0], self.0[1], self.0[2]))
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self([x, y, z, 1])
    }

    fn distance(&self, other: &Self) -> usize {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(l, r)| (l - r).abs() as usize)
            .sum()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Matrix([[isize; 4]; 4]);

#[derive(Clone, Copy, Debug)]
struct IterCol<'a> {
    matrix: &'a Matrix,
    col: usize,
    next_idx: usize,
}

impl<'a> Iterator for IterCol<'a> {
    type Item = &'a isize;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.next_idx;
        self.next_idx += 1;
        self.matrix.0.get(idx).map(|row| &row[self.col])
    }
}

impl<'a> IterCol<'a> {
    fn new(matrix: &'a Matrix, col: usize) -> Self {
        IterCol {
            matrix,
            col,
            next_idx: 0,
        }
    }
}

impl<'l, 'r> std::ops::Mul<&'r Matrix> for &'l Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'r Matrix) -> Self::Output {
        let m = |row: usize, col: usize| -> isize {
            self.iter_row(row)
                .zip(rhs.iter_col(col))
                .map(|(l, r)| *l * *r)
                .sum()
        };
        Matrix([
            [m(0, 0), m(0, 1), m(0, 2), m(0, 3)],
            [m(1, 0), m(1, 1), m(1, 2), m(1, 3)],
            [m(2, 0), m(2, 1), m(2, 2), m(2, 3)],
            [m(3, 0), m(3, 1), m(3, 2), m(3, 3)],
        ])
    }
}

impl<'l, 'r> std::ops::Mul<&'r Vector> for &'l Matrix {
    type Output = Vector;

    fn mul(self, rhs: &'r Vector) -> Self::Output {
        let r = &rhs.0;
        Vector([
            self.iter_row(0).zip(r.iter()).map(|(l, r)| *l * *r).sum(),
            self.iter_row(1).zip(r.iter()).map(|(l, r)| *l * *r).sum(),
            self.iter_row(2).zip(r.iter()).map(|(l, r)| *l * *r).sum(),
            self.iter_row(3).zip(r.iter()).map(|(l, r)| *l * *r).sum(),
        ])
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])
    }
}

impl Matrix {
    fn new(matrix: [[isize; 4]; 4]) -> Self {
        Self(matrix)
    }

    fn with_x_rot90() -> Self {
        Self::new([[1, 0, 0, 0], [0, 0, -1, 0], [0, 1, 0, 0], [0, 0, 0, 1]])
    }

    fn with_y_rot90() -> Self {
        Self::new([[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]])
    }

    fn with_z_rot90() -> Self {
        Self::new([[0, -1, 0, 0], [1, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &isize> {
        self.0[row].iter()
    }

    fn iter_col(&self, col: usize) -> impl Iterator<Item = &isize> {
        IterCol::new(self, col)
    }

    fn with_matching_translation(&self, from: &Vector, to: &Vector) -> Self {
        let mut result = *self;
        let vec = self * from;
        for i in 0..3 {
            result.0[i][3] = to.0[i] - vec.0[i];
        }
        result
    }
}

fn parse(input: &str) -> Vec<Vec<Vector>> {
    let mut scanners = Vec::new();
    let mut current_scanner = Vec::new();
    for line in input.lines() {
        if line.starts_with("---") {
            if !current_scanner.is_empty() {
                scanners.push(current_scanner);
                current_scanner = Vec::new();
            }
        } else if !line.is_empty() {
            let mut nums = line.split(',').map(|s| s.parse::<isize>().unwrap());
            current_scanner.push(Vector::new(
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            ));
        }
    }
    if !current_scanner.is_empty() {
        scanners.push(current_scanner);
    }
    scanners
}

static POSSIBLE_ROTATIONS: Lazy<[Matrix; 24]> = Lazy::new(|| {
    let mut rotations = vec![Matrix::default()];
    for _ in 0..4 {
        for new_rotations in [
            rotations
                .iter()
                .map(|r| r * &Matrix::with_x_rot90())
                .collect::<Vec<Matrix>>(),
            rotations
                .iter()
                .map(|r| r * &Matrix::with_y_rot90())
                .collect::<Vec<Matrix>>(),
            rotations
                .iter()
                .map(|r| r * &Matrix::with_z_rot90())
                .collect::<Vec<Matrix>>(),
        ]
        .iter_mut()
        {
            rotations.append(new_rotations);
        }
    }
    rotations
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

#[derive(Clone, Debug, Default)]
struct MatchedReadings {
    transformation: Matrix,
    unmatched: Vec<Vector>,
}

fn match_readings(base_readings: &HashSet<Vector>, readings: &[Vector]) -> MatchedReadings {
    let mut best_match = MatchedReadings {
        transformation: Matrix::default(),
        unmatched: base_readings.iter().cloned().collect(),
    };
    for base_vec in base_readings {
        for vec in readings {
            for rot in *POSSIBLE_ROTATIONS {
                let rev_transformation = rot.with_matching_translation(vec, base_vec);
                let unmatched: Vec<Vector> = readings
                    .iter()
                    .map(|v| &rev_transformation * v)
                    .filter(|v| !base_readings.contains(v))
                    .collect();
                if unmatched.len() < best_match.unmatched.len() {
                    best_match = MatchedReadings {
                        transformation: rev_transformation,
                        unmatched,
                    };
                }
            }
        }
    }
    best_match
}

// very crap solution - repeats a lot of calculations over and over again (full solution takes almost 30mins)
fn match_all(readings: &[Vec<Vector>]) -> (HashSet<Vector>, Vec<Matrix>) {
    let mut transformations = vec![None; readings.len()];
    let mut absolute_readings = HashSet::from_iter(readings.first().unwrap().iter().cloned());
    let mut matched_readings = HashSet::new();
    matched_readings.insert(0);
    transformations[0] = Some(Matrix::default());

    loop {
        let mut best: Option<(usize, Vec<Vector>, Matrix)> = None;
        for (idx, readings) in readings.iter().enumerate() {
            if !matched_readings.contains(&idx) {
                let candidate = match_readings(&absolute_readings, readings);
                if best
                    .as_ref()
                    .map(|(_, best_unmatched, _)| candidate.unmatched.len() < best_unmatched.len())
                    .unwrap_or(true)
                {
                    best = Some((idx, candidate.unmatched, candidate.transformation));
                }
            }
        }
        if let Some((best_idx, best_unmatched, best_transformation)) = best {
            matched_readings.insert(best_idx);
            absolute_readings.extend(best_unmatched.into_iter());
            transformations[best_idx] = Some(best_transformation);
            println!("{}/{}", matched_readings.len(), readings.len());
        } else {
            break;
        }
    }
    (
        absolute_readings,
        transformations
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .unwrap(),
    )
}

fn biggest_distance(transformations: &[Matrix]) -> usize {
    let points = transformations
        .iter()
        .map(|m| m * &Vector::default())
        .collect::<Vec<Vector>>();
    let mut max_distance = 0;
    for p1 in &points {
        for p2 in &points {
            let d = p1.distance(p2);
            if max_distance < d {
                max_distance = d;
            }
        }
    }
    max_distance
}

fn main() {
    let readings = parse(include_str!("../../inputs/day19.txt"));
    let (absolute_readings, transformations) = match_all(&readings);
    println!("Part 1: {}", absolute_readings.len());
    println!("Part 2: {}", biggest_distance(&transformations));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("--- scanner 0 ---\n404,-588,-901\n528,-643,409\n-838,591,734\n\n--- scanner 1 ---\n686,422,578\n605,423,415\n515,917,-361\n\n--- scanner 2 ---\n649,640,665\n682,-795,504\n-784,533,-524"),
            vec![
                vec![
                    Vector::new(404,-588,-901),
                    Vector::new(528,-643,409),
                    Vector::new(-838,591,734),
                ],
                vec![
                    Vector::new(686,422,578),
                    Vector::new(605,423,415),
                    Vector::new(515,917,-361),
                ],
                vec![
                    Vector::new(649,640,665),
                    Vector::new(682,-795,504),
                    Vector::new(-784,533,-524),
                ],
            ]
        );
    }

    #[test]
    fn test_matrix_mul_matrix() {
        assert_eq!(&Matrix::default() * &Matrix::default(), Matrix::default());
    }

    #[test]
    fn test_matrix_mul_vector() {
        assert_eq!(
            &Matrix::default() * &Vector::new(1, 2, 3),
            Vector::new(1, 2, 3)
        );
    }

    #[test]
    fn test_translation_matching() {
        let a = Vector::new(0, 0, 0);
        let b = Vector::new(10, 20, 30);
        let translation = Matrix::default().with_matching_translation(&a, &b);
        assert_eq!(&translation * &a, b);
    }

    #[test]
    fn test_possible_rotations() {
        assert_eq!(
            POSSIBLE_ROTATIONS
                .iter()
                .cloned()
                .collect::<HashSet<Matrix>>()
                .len(),
            POSSIBLE_ROTATIONS.len()
        );
    }

    #[test]
    #[ignore]
    fn test_sample() {
        let readings = parse(include_str!("../../inputs/day19-sample.txt"));
        let (absolute_readings, transformations) = match_all(&readings);
        assert_eq!(absolute_readings.len(), 79);
        assert_eq!(biggest_distance(&transformations), 3621);
    }
}

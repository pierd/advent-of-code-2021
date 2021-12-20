use std::fmt::Write;

fn parse(input: &str) -> (EnhanceVec, Image) {
    let mut lines = input.lines();
    let enhance = lines.next().unwrap().chars().map(|c| c == '#').into();

    lines.next(); // skip empty line

    let mut pixels = Vec::new();
    while let Some(line) = lines.next() {
        pixels.push(line.chars().map(|c| c == '#').collect());
    }

    (
        enhance,
        Image {
            pixels,
            flipped: false,
        },
    )
}

#[derive(Clone, Default)]
struct Image {
    pixels: Vec<Vec<bool>>,
    flipped: bool,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for row in &self.pixels {
            for b in row {
                f.write_char(if self.flipped ^ *b { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Image {
    fn with_rows_cols(rows: usize, cols: usize, flipped: bool) -> Self {
        let pixels = vec![vec![false; cols]; rows];
        Self { pixels, flipped }
    }

    fn rows(&self) -> usize {
        self.pixels.len()
    }

    fn cols(&self) -> usize {
        self.pixels.first().map(Vec::len).unwrap_or_default()
    }

    fn get_pixel(&self, row: isize, col: isize) -> bool {
        if row >= 0 && col >= 0 {
            let row = row as usize;
            let col = col as usize;
            if row < self.rows() && col < self.cols() {
                self.pixels[row][col] ^ self.flipped
            } else {
                self.flipped
            }
        } else {
            self.flipped
        }
    }

    fn set_pixel(&mut self, row: usize, col: usize, lit: bool) {
        self.pixels[row][col] = lit ^ self.flipped;
    }

    const ENHANCE_ORDER: &'static [(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn get_enhanced(&self, enhance: &EnhanceVec, row: isize, col: isize) -> bool {
        let mut enhance_idx = 0;
        for (drow, dcol) in Self::ENHANCE_ORDER {
            enhance_idx <<= 1;
            enhance_idx |= if self.get_pixel(row + drow, col + dcol) {
                1
            } else {
                0
            };
        }
        enhance.0[enhance_idx]
    }

    fn enhanced_with(&self, enhance: &EnhanceVec) -> Self {
        let new_rows = self.rows() + 2;
        let new_cols = self.cols() + 2;
        let flipping = *enhance.0.first().unwrap() && !*enhance.0.last().unwrap();
        let mut new = Self::with_rows_cols(new_rows, new_cols, flipping ^ self.flipped);
        for row in 0..new_rows {
            for col in 0..new_cols {
                new.set_pixel(
                    row,
                    col,
                    self.get_enhanced(enhance, row as isize - 1, col as isize - 1),
                );
            }
        }
        new
    }

    fn enhanced_times_with(&self, enhance: &EnhanceVec, times: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..times {
            result = result.enhanced_with(&enhance);
        }
        result
    }

    fn count_lit(&self) -> usize {
        assert!(!self.flipped);
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .map(|p| if *p { 1 } else { 0 })
            .sum()
    }
}

#[derive(Clone, Debug)]
struct EnhanceVec(Vec<bool>);

impl<Iter> From<Iter> for EnhanceVec
where
    Iter: Iterator<Item = bool>,
{
    fn from(iter: Iter) -> Self {
        Self(iter.collect())
    }
}

fn main() {
    let (enhance, image) = parse(include_str!("../../inputs/day20.txt"));
    println!(
        "Part 1: {}",
        image.enhanced_times_with(&enhance, 2).count_lit()
    );
    println!(
        "Part 2: {}",
        image.enhanced_times_with(&enhance, 50).count_lit()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample() {
        let (enhance, image) = parse(include_str!("../../inputs/day20-sample.txt"));
        assert_eq!(enhance.0.len(), 512);
        assert_eq!(
            enhance
                .0
                .iter()
                .map(|b| if *b { 1 } else { 0 })
                .sum::<usize>(),
            238
        );
        assert_eq!(image.count_lit(), 10);
    }

    #[test]
    fn test_sample() {
        let (enhance, image) = parse(include_str!("../../inputs/day20-sample.txt"));
        assert_eq!(
            image
                .enhanced_with(&enhance)
                .enhanced_with(&enhance)
                .count_lit(),
            35
        );
    }
}

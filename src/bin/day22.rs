use std::{
    collections::{btree_map, BTreeMap},
    iter::repeat,
};

type Int = isize;

fn parse(input: &str) -> Vec<(bool, [(Int, Int); 3])> {
    input
        .lines()
        .map(|line| {
            let (first, rest) = line.split_once(' ').unwrap();
            let mut pairs = rest
                .split(',')
                .map(|part| part.split_at(2).1.split_once("..").unwrap())
                .map(|(from, to)| (from.parse::<Int>().unwrap(), to.parse::<Int>().unwrap()));
            (
                first == "on",
                [
                    pairs.next().unwrap(),
                    pairs.next().unwrap(),
                    pairs.next().unwrap(),
                ],
            )
        })
        .collect()
}

fn solve_simple(cubes: &[(bool, [(Int, Int); 3])]) -> usize {
    let mut space = vec![vec![vec![false; 101]; 101]; 101];
    for (on, cube) in cubes {
        if 50 < cube[0].0 || cube[0].0 < -50 {
            continue;
        }
        for x in cube[0].0..=cube[0].1 {
            for y in cube[1].0..=cube[1].1 {
                for z in cube[2].0..=cube[2].1 {
                    space[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = *on;
                }
            }
        }
    }
    space
        .into_iter()
        .flat_map(|v| v.into_iter())
        .flat_map(|v| v.into_iter())
        .filter(|p| *p)
        .count()
}

fn fork_from_prev<K, V>(tree: &mut BTreeMap<K, V>, key: K)
where
    K: Ord + Clone,
    V: Clone,
{
    if let Some((prev_key, prev_val)) = tree.range(..=key.clone()).next_back() {
        if prev_key < &key {
            // no entry -> create one from previous entry
            let forked = prev_val.clone();
            tree.insert(key, forked);
        }
    }
}

fn fork_and_mut<K, V, F>(tree: &mut BTreeMap<K, V>, from: K, to: K, transform: F)
where
    K: Ord + Clone,
    V: Clone,
    F: Fn(&mut V),
{
    assert!(from < to);
    fork_from_prev(tree, from.clone());
    fork_from_prev(tree, to.clone());
    for (_, v) in tree.range_mut(from..to) {
        transform(v);
    }
}

struct BTreeMapRangeIter<'a, K, V> {
    iter: btree_map::Iter<'a, K, V>,
    prev_key: Option<&'a K>,
    prev_val: Option<&'a V>,
}

impl<'a, K, V> BTreeMapRangeIter<'a, K, V> {
    fn with_btreemap(tree: &'a BTreeMap<K, V>) -> Self {
        let mut iter = tree.iter();
        if let Some((prev_key, prev_val)) = iter.next() {
            Self {
                iter,
                prev_key: Some(prev_key),
                prev_val: Some(prev_val),
            }
        } else {
            Self {
                iter,
                prev_key: None,
                prev_val: None,
            }
        }
    }
}

impl<'a, K, V> Iterator for BTreeMapRangeIter<'a, K, V> {
    type Item = (&'a K, &'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(next_key, next_val)| {
            // prev_key and prev_val are never None as long as iter has elements
            let prev_key = self.prev_key.replace(next_key).unwrap();
            let val = self.prev_val.replace(next_val).unwrap();
            (prev_key, next_key, val)
        })
    }
}

fn solve(cubes: &[(bool, [(Int, Int); 3])]) -> Int {
    let mut space = {
        let mut empty_z: BTreeMap<Int, bool> = Default::default();
        empty_z.insert(Int::MIN, false);
        let mut empty_yz: BTreeMap<Int, BTreeMap<Int, bool>> = Default::default();
        empty_yz.insert(Int::MIN, empty_z);
        let mut empty_xyz: BTreeMap<Int, BTreeMap<Int, BTreeMap<Int, bool>>> = Default::default();
        empty_xyz.insert(Int::MIN, empty_yz);
        empty_xyz
    };

    for (on, cube) in cubes {
        fork_and_mut(&mut space, cube[0].0, cube[0].1 + 1, |subspace| {
            fork_and_mut(subspace, cube[1].0, cube[1].1 + 1, |subspace| {
                fork_and_mut(subspace, cube[2].0, cube[2].1 + 1, |b| *b = *on);
            });
        });
    }

    BTreeMapRangeIter::with_btreemap(&space)
        .flat_map(|(x1, x2, subspace)| {
            repeat((x1, x2)).zip(BTreeMapRangeIter::with_btreemap(subspace))
        })
        .flat_map(|((x1, x2), (y1, y2, subspace))| {
            repeat((x1, x2, y1, y2)).zip(BTreeMapRangeIter::with_btreemap(subspace))
        })
        .map(|((x1, x2, y1, y2), (z1, z2, b))| {
            if *b {
                (x2 - x1) * (y2 - y1) * (z2 - z1)
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let parsed = parse(include_str!("../../inputs/day22.txt"));
    println!("Part 1: {}", solve_simple(&parsed));
    println!("Part 2: {}", solve(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample() {
        let parsed = parse(include_str!("../../inputs/day22-sample.txt"));
        assert_eq!(parsed.len(), 22);
        assert_eq!(parsed[0], (true, [(-20, 26), (-36, 17), (-47, 7)]));
        assert_eq!(parsed[10], (false, [(-48, -32), (26, 41), (-47, -37)]));
    }

    #[test]
    fn test_sample() {
        let parsed = parse(include_str!("../../inputs/day22-sample.txt"));
        assert_eq!(solve_simple(&parsed), 590784);
    }

    #[test]
    fn test_sample2() {
        let parsed = parse(include_str!("../../inputs/day22-sample2.txt"));
        assert_eq!(solve(&parsed), 2758514936282235);
    }

    #[test]
    fn test_fork_and_mut() {
        let mut tree: BTreeMap<isize, bool> = Default::default();
        tree.insert(isize::MIN, false);
        fork_and_mut(&mut tree, 0, 10, |b| *b = true);
        fork_and_mut(&mut tree, 5, 10, |b| *b = false);
        fork_and_mut(&mut tree, 7, 15, |b| *b = true);
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some((isize::MIN, false)));
        assert_eq!(iter.next(), Some((0, true)));
        assert_eq!(iter.next(), Some((5, false)));
        assert_eq!(iter.next(), Some((7, true)));
        assert_eq!(iter.next(), Some((10, true)));
        assert_eq!(iter.next(), Some((15, false)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_range() {
        let mut tree: BTreeMap<isize, bool> = Default::default();
        tree.insert(isize::MIN, false);
        fork_and_mut(&mut tree, 0, 10, |b| *b = true);
        fork_and_mut(&mut tree, 5, 10, |b| *b = false);
        fork_and_mut(&mut tree, 7, 15, |b| *b = true);
        let mut iter = BTreeMapRangeIter::with_btreemap(&tree);
        assert_eq!(iter.next(), Some((&isize::MIN, &0, &false)));
        assert_eq!(iter.next(), Some((&0, &5, &true)));
        assert_eq!(iter.next(), Some((&5, &7, &false)));
        assert_eq!(iter.next(), Some((&7, &10, &true)));
        assert_eq!(iter.next(), Some((&10, &15, &true)));
        assert_eq!(iter.next(), None);
    }
}

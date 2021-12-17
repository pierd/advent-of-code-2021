use std::ops::RangeBounds;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Status {
    highest: isize,
    reached_target: bool,
}

fn unwrap_bound<T>(bound: std::ops::Bound<T>) -> T {
    match bound {
        std::ops::Bound::Included(x) => x,
        std::ops::Bound::Excluded(x) => x,
        _ => unreachable!(),
    }
}

fn simulate<XRange: RangeBounds<isize>, YRange: RangeBounds<isize>>(
    mut vx: isize,
    mut vy: isize,
    tx: &XRange,
    ty: &YRange,
) -> Status {
    let mut x = 0;
    let mut y = 0;
    let mut highest = 0;
    let mut reached_target = false;
    let deepest = *unwrap_bound(ty.start_bound());
    let max_x = *unwrap_bound(tx.end_bound());
    while !reached_target && !(vx == 0 && (y < deepest || !tx.contains(&x))) && x <= max_x {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
        if highest < y {
            highest = y;
        }
        if tx.contains(&x) && ty.contains(&y) {
            reached_target = true;
        }
    }
    Status {
        highest,
        reached_target,
    }
}

fn find_highest<XRange: RangeBounds<isize>, YRange: RangeBounds<isize>>(
    tx: &XRange,
    ty: &YRange,
) -> isize {
    let max_x = *unwrap_bound(tx.end_bound());
    let mut max_highest = 0;
    for vx in 0..=max_x {
        for vy in 0..1000 {
            if let Status {
                highest,
                reached_target: true,
            } = simulate(vx, vy, tx, ty)
            {
                if max_highest < highest {
                    max_highest = highest;
                }
            }
        }
    }
    max_highest
}

fn count_within<XRange: RangeBounds<isize>, YRange: RangeBounds<isize>>(
    tx: &XRange,
    ty: &YRange,
) -> usize {
    let max_x = *unwrap_bound(tx.end_bound());
    let min_y = *unwrap_bound(ty.start_bound());
    let mut count = 0;
    for vx in 0..=max_x {
        for vy in min_y..1000 {
            if simulate(vx, vy, tx, ty).reached_target {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", find_highest(&(32..=65), &(-225..=-177)));
    println!("Part 2: {}", count_within(&(32..=65), &(-225..=-177)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        assert_eq!(
            simulate(7, 2, &(20..=30), &(-10..=-5)),
            Status {
                highest: 3,
                reached_target: true
            }
        );
        assert_eq!(
            simulate(6, 3, &(20..=30), &(-10..=-5)),
            Status {
                highest: 6,
                reached_target: true
            }
        );
        assert_eq!(
            simulate(9, 0, &(20..=30), &(-10..=-5)),
            Status {
                highest: 0,
                reached_target: true
            }
        );
        assert_eq!(
            simulate(17, -4, &(20..=30), &(-10..=-5)),
            Status {
                highest: 0,
                reached_target: false
            }
        );
        assert_eq!(
            simulate(6, 9, &(20..=30), &(-10..=-5)),
            Status {
                highest: 45,
                reached_target: true
            }
        );
    }

    #[test]
    fn find_highest_sample() {
        assert_eq!(find_highest(&(20..=30), &(-10..=-5)), 45);
    }

    #[test]
    fn count_within_sample() {
        assert_eq!(count_within(&(20..=30), &(-10..=-5)), 112);
    }
}

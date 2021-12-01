use itertools::Itertools;

use super::*;

type IntTup = (i32, i32);

fn add_if_larger(acc: i32, (prev, next): IntTup) -> i32 {
    acc + if next > prev { 1 } else { 0 }
}

pub fn part1() {
    let res = get_text("day1")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .tuple_windows::<IntTup>()
        .fold(0, add_if_larger);
    println!("{}", res);
}

pub fn part2() {
    let res = get_text("day1")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec()
        .windows(3)
        .map(|x| x.iter().sum())
        .tuple_windows::<IntTup>()
        .fold(0, add_if_larger);
    println!("{}", res);
}

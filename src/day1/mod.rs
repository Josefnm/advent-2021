pub fn part1() -> String {
    count_increments(1)
}

pub fn part2() -> String {
    count_increments(3)
}


fn count_increments(win_size: usize) -> String {
    include_str!("input.txt").lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>()
        .windows(win_size)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|a| a[0] < a[1])
        .count()
        .to_string()
}

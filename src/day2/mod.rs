pub fn part1() -> String {
    let mover = |(x, y, z), (direction, n)|
        match direction {
            "up" => (x - n, y, z),
            "down" => (x + n, y, z),
            "forward" => (x, y + n, z),
            _ => (x, y, z)
        };
    solver(mover)
}

pub fn part2() -> String {
    let mover = |(horizontal, depth, aim), (direction, n)|
        match direction {
            "up" => (horizontal, depth, aim - n),
            "down" => (horizontal, depth, aim + n),
            "forward" => (horizontal + n, depth + aim * n, aim),
            _ => (horizontal, depth, aim),
        };
    solver(mover)
}

type Result = (i32, i32, i32);

type Command = (&'static str, i32);

fn solver(solver_fn: fn(Result, Command) -> Result) -> String {
    let (x, y, _) = include_str!("input.txt")
        .trim()
        .lines()
        .map(|s|
            s.split(' ').collect::<Vec<&str>>())
        .map(|a| (a[0], a[1].parse::<i32>().unwrap()))
        .fold((0, 0, 0), solver_fn);
    (x * y).to_string()
}


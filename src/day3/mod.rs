use std::borrow::Borrow;
use std::cmp::Ordering;

pub fn part1() -> String {
    let input = include_str!("input.txt").lines().collect::<Vec<_>>();

    let length = input.first().unwrap().len();
    let x = input
        .iter()
        .map(|item|
            item.chars()
                .map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        )
        .fold(vec![0_u32; length], |prev, next|
            prev.iter()
                .zip(next)
                .map(|(a, b)| a + b.unwrap())
                .collect::<Vec<u32>>(),
        )
        .iter()
        .map(|n| n * 2 / input.len() as u32)
        .collect::<Vec<_>>();
    let gamma = binary_vec_to_int(x);
    let epsilon = 2_isize.pow(length as u32) - 1 - gamma;
    (gamma * epsilon).to_string()
}

pub fn part2() -> String {
    let input = include_str!("input.txt").lines().collect::<Vec<_>>();

    let oxygen = binary_vec_to_int(find_answer(&input, 1));
    let co2 = binary_vec_to_int(find_answer(&input, 0));

    (oxygen * co2).to_string()
}

fn binary_vec_to_int(vals: Vec<u32>) -> isize {
    let x = vals.iter().fold("".to_string(), |prev, next|
        prev + &next.to_string(),
    );
    isize::from_str_radix(x.borrow(), 2).unwrap()
}

fn find_answer(input: &[&str], binary: usize) -> Vec<u32> {
    let mut x = to_2d_vec(input);


    for i in 0..x.len() {
        let split = split_by_value(&x, i);

        let chosen_vec_i = match split[0].len().cmp(&split[1].len()) {
            Ordering::Greater => 1 ^ binary,
            _ => binary,
        };
        x = split[chosen_vec_i].to_owned();
        if x.len() == 1 {
            return x.first().unwrap().to_owned();
        }
    }
    panic!("no result")
}

fn to_2d_vec(input: &[&str]) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|item|
            item.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
        )
        .collect::<Vec<Vec<u32>>>()
}


fn split_by_value(x: &[Vec<u32>], i: usize) -> Vec<Vec<Vec<u32>>> {
    x.iter()
        .fold(
            vec![Vec::new(), Vec::new()],
            |mut acc: Vec<Vec<Vec<u32>>>, y| {
                acc[y[i] as usize].push(y.to_owned());
                acc
            },
        )
}

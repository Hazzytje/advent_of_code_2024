use std::fs;
use std::iter::zip;
use std::collections::HashMap;

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Reading input file error");
    let input = input.lines();

    let mut list1: Vec<i32> = input
        .clone() //TODO hm
        .map(|l| l.split("   ").next().unwrap().parse().unwrap())
        .collect();

    let mut list2: Vec<i32> = input
        .map(|l| l.split("   ").skip(1).next().unwrap().parse().unwrap())
        .collect();

    list1.sort();
    list2.sort();

    let sum_of_distances: u32 = zip(list1, list2).map(|(a, b)| a.abs_diff(b)).sum();
    println!("sum of differences: {}", sum_of_distances);
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("Reading input file error");
    let input = input.lines();

    let list1: Vec<i32> = input
        .clone() //TODO hm
        .map(|l| l.split("   ").next().unwrap().parse().unwrap())
        .collect();

    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in input .map(|l| l.split("   ").skip(1).next().unwrap().parse().unwrap())
    {
        *map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = list1.iter().map(|a| a * map.get(a).unwrap_or(&0)).sum();
    println!("similarity: {}", similarity_score);
}

fn main() {
    part1();
    part2();
}

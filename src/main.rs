use std::io;

fn main() {
    println!("Welcome to the calculator");

    println!("Please enter what operation you want to run");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Invalid input");

    if let "average" = &*input.trim() {
        println!("Enter numbers you find to find the average of");
        let vec_n = get_vec_from_input();
        let avg = get_average(&vec_n);
        println!("The average of the input numbers is {}", avg);
    } else if let "median" = &*input.trim() {
        println!("Enter number you want to find median of");
        let vec_n = get_vec_from_input();
        let median = get_median(&vec_n);
        println!("The median is {}", median);
    } else if let "mode" = &*input.trim() {
        println!("Enter the numbers you want to find the mode of");
        let vec_n = get_vec_from_input();
        let mode = get_mode(&vec_n);
        println!("The mode is {}", mode);
    }
}

fn get_average(nums: &Vec<i32>) -> f64 {
    let mut s = 0;
    for number in nums {
        s += *number
    }
    (s as f64) / (nums.len() as f64)
}

fn get_median(nums: &Vec<i32>) -> i32 {
    let mut clone = nums.clone();
    clone.sort();
    match clone.get(clone.len() / 2) {
        Some(val) => *val,
        None => {
            panic!("IMPOSSIBLE");
        }
    }
}
use std::collections::HashMap;
fn get_mode(nums: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    let mut ret = 0;
    for number in nums {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
            ret = *number;
        }
    }
    ret
}

fn get_vec_from_input() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Invalid input");
    let iter = s.trim().split_whitespace();
    let vec_n: Vec<i32> = iter
        .map(|s: &str| s.parse().expect("Not a number"))
        .collect();
    vec_n
}

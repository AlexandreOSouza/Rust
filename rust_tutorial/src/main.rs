
#![allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp:: Ordering;


fn sum(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val
    }
    return sum;
}

fn main() {

    let list: Vec<i32> = vec![1, 2, 3, 4, 5];
        println!("Sum: {}", sum(&list));
}


#![allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp:: Ordering;


fn main() {

    let my_tuple: (u8,String, f64) = (47, "Alexandre".to_string(), 50_000.00);

    println!("Name: {}", my_tuple.1);
    println!("Age: {}, ", my_tuple.0);
}

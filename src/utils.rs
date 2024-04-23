use std::fs::{ File};
use std::io::{BufReader, BufRead};

pub fn ex_file()-> BufReader<File>{
    let file = File::open("./excercice.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
}

pub fn factorial(n:i32)->i32 {
    let mut fact = 1;
    for i in 0..n{
        fact *= i+1;
    }
    return fact;
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input() -> std::string::String {
    let path = Path::new("../1.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open the file {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}:  {}", display, why.description()),
        Ok(_) => ()
    };
    s
}

fn main() {

    let s = read_input();
    let split = s.split_whitespace();
    let mut result = 0;
    let mut frequency = 0;
    let mut buf: Vec<i32> = Vec::new();
    for s1 in split {
        result += s1.parse::<i32>().unwrap();
        if buf.contains(&result)
        {
            frequency = result;
        }
        buf.push(result);
    }

    let num: Vec<i32>  = s.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

//    result = num.iter().fold(0i32, |acc, &x| { let y = acc + x; buf.push(y); y});
  //  result = num.iter().sum();
    let map = result;
    for s1 in s.split_whitespace().cycle() {
        result += s1.parse::<i32>().unwrap();
        if buf.contains(&result)
        {
            frequency = result;
            break;
        }
        buf.push(result);
    }
    println!("result: 1 {} 2 {}", map, frequency);
}

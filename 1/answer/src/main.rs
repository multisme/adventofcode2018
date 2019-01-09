use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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
    let split = s.split_whitespace();
    let mut result = 0;
    let mut frequency = 0;
    let mut buf: Vec<i32> = Vec::new();
    buf.push(result);
    for s1 in split {
        result += s1.parse::<i32>().unwrap();
        if buf.contains(&result)
        {
            println!("well mister bond");
            frequency = result;
        }
        buf.push(result);
    }
    let num: Vec<i32>  = s.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // result = num.iter().fold(0i32, |acc, &x| acc + x);
    result = num.iter().sum();
    let map = result;
    while frequency == 0 {
        for s1 in s.split_whitespace() {
            result += s1.parse::<i32>().unwrap();
            if buf.contains(&result)
            {
                frequency = result;
                println!("well mister bond");
                break;
            }
            buf.push(result);
        }
    };
    println!("result {} {}", map, frequency);
}

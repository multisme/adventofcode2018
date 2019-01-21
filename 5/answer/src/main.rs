use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;
use std::char;

fn read_input() -> std::string::String {
    let path = Path::new("../5.txt");
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

fn polymer(mut data: String, spliters: &Vec<String>) -> usize{

    let mut len1 = data.len();
    loop {
        for split in spliters.clone(){
            data = data.split(&split).collect::<Vec<_>>().concat();        
        }
        let len2 = data.len();
        if len1 == len2{
            break;
        }
        len1 = len2;
    }
    len1
}

fn main() {

    let s: String = read_input();
    // let mut data = s.lines().collect::<Vec<_>>();

    let mut spliters =  Vec::new();

    for x in 65..91{
        let mut vec_c : String = [char::from_u32(x).unwrap(),
        char::from_u32(x + 32).unwrap()].iter().collect();
        spliters.push(vec_c);
        let mut vec_c : String = [char::from_u32(x).unwrap(),
        char::from_u32(x + 32).unwrap()].iter().rev().collect();
        spliters.push(vec_c);
    }
    let data = s.clone();
    let result1 = polymer(data, &spliters) - 1;
    let mut result2 = result1;
    for x in 65..91{
        let mut data = s.clone();
        let c1: char = char::from_u32(x).unwrap();
        let c2: char = char::from_u32(x + 32).unwrap();
        data.retain(|c| c != c1 && c != c2);
        result2 = cmp::min(result2, polymer(data, &spliters) - 1);
    }
    println!("{:?} {:?}", result1 - 1, result2);
}

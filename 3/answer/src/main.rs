use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
extern crate edit_distance;

fn read_input() -> std::string::String {
    let path = Path::new("../3.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open the file {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}:  {}", display, why.description()),
        Ok(_) => (),
    };
    s
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let data = line.split_whitespace().collect::<Vec<_>>();
    let coord = data[2]
        .split(&[',', ':'][0..])
        .take(2)
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let size = data[3]
        .split('x')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    ((coord[0], coord[1]), (size[0], size[1]))
}

fn draw_squares(map: &mut Vec<u32>, square: ((u32, u32), (u32, u32))) -> bool {
    let mut ret = true;
    let y: u32 = (square.0).1 * 1000;
    for j in 0..(square.1).1 {
        let pos: u32 = y + 1000 * j + (square.0).0 as u32;
        for i in 0..(square.1).0 {
            let index: u32 = pos + i;
            if map[index as usize] > 1 {
                ret = false;
            }
            map[index as usize] += 1;
        }
    }
    ret
}

fn main() {
    let s = read_input();
    let mut vec = vec![0; 1000000];
    for s1 in s.lines() {
        draw_squares(&mut vec, parse_line(s1));
    }
    let mut result2 = "";
    for s1 in s.lines() {
        if draw_squares(&mut vec, parse_line(s1)) {
            result2 = s1;
        }
    }
    let result1 = vec.iter().filter(|&x| *x > 1).count();
    println!("result1: 1 {} 2 {:?}", result1, result2);
}

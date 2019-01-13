use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
extern crate edit_distance;

fn read_input() -> std::string::String {
    let path = Path::new("../2.txt");
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

pub fn tmetrics_hamming(a: &[u16], b: &[u16]) -> usize {
    a.iter().zip(b).filter(|&(a, b)| a == b).count()
}

fn main() {
 
    let s = read_input();
    let mut result1 = 0;
    let mut result2 = 0;
    let len = s.split_whitespace()
        .next().unwrap().len() + 1;
    let mut temp = vec![0; len];;

    for x in s.split_whitespace(){
        let mut char_c: HashMap<char, u32> = HashMap::new();
        for c in x.chars(){
            let count = char_c.entry(c).or_insert(0);
            *count += 1;
        }

        let mut call = char_c.values()
            .filter(|&v| *v > 1)
            .collect::<Vec<_>>();
        call.sort_unstable();
        call.dedup();
        for d in call{
            temp[*d as usize] += 1;
        }
    }
    result1 = temp.iter().filter(|&v| *v != 0).product();
    let mut temp2 = Vec::new();
    for d in s.split_whitespace(){
        for b in s.split_whitespace(){
            if (b != d){
                let res = b.chars().zip(d.chars())
                    .filter(|&(c1, c2)| c1 == c2)
                    .collect::<Vec<_>>();
                temp2.push(res);
            }
        }
    }
    temp2.sort_by(|a, b| b.len().cmp(&a.len()));
  //  let mut result2 = s.split_whitespace().collect::<Vec<_>>()
  //      .sort_by(|a, b| edit_distance::edit_distance(a, b));
    let result2 = temp2.first().into_iter().collect::<Vec<_>>();
    //println!("{:?}", temp2.first().map(|&(s1,_)| s1).collect::str());
    println!("result1: 1 {} 2 {:?}", result1, result2);
}

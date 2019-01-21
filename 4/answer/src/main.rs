use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn read_input() -> std::string::String {
    let path = Path::new("../4.txt");
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

fn manage_time(str: &str, p: &mut Vec<i16>, val: i16)->(){

    let time: Vec<usize> = str.split(&[':', ']'][..])
        .take(2).map(|x| x.parse().unwrap())
        .collect();

    let index =
        if time[0] == 23 { 0 }
        else if time[0] == 00 { time[1] }
        else { 60 };

    for x in index..60{ p[x] += val}
}

fn main() {

    let s = read_input();
    let mut data = s.lines().collect::<Vec<_>>();
    let mut map: HashMap<String, Vec<_>> = HashMap::new();

    // Sort the line in timely fashion
    data.sort();
    let data = data.join("\n");

    // Split the data un shifts
    let mut insert = String::new();
    for line in data.lines(){
        if line.contains("Guard"){
            insert.push_str("*");
        }
        insert.push_str(line);
        insert.push_str("\n");
    }
    let mut data = insert.split("*");

    // Remove first empty string
    data.next();

    // Parse Shifts
    for shift in data{
        let mut shift = shift.lines();
        let mut splited = shift.next()
            .unwrap().split_whitespace()
            .collect::<Vec<_>>();
        let p = map.entry(splited[3].to_string())
            .or_insert(vec![0i16; 60]);
        manage_time(splited[1], p, 0);
        for line in shift{
            let mut splited = line
                .split_whitespace();
            let split = splited.nth(1).unwrap();
            if line.contains("wakes"){
                manage_time(split, p, -1);
            } else {
                manage_time(split, p, 1);
            }
        }
    }

    // Compute all the shifts  
    let result = map.iter()
        .map(|(k, v)|   (k, v.iter().sum::<i16>(), v.iter()
                         .position(|r| r == v.iter().max().unwrap()).unwrap(),
                         v.iter().max().unwrap()
                        ))
        .collect::<Vec<_>>();

    // Compute the results
    let result1 = result.iter()
        .max_by_key(|(_k, v, _s, _t)| v).unwrap();
    let result1 = (result1.0)[1..].parse::<i32>().unwrap() * result1.2 as i32;

    let result2 = result.iter()
        .max_by_key(|(_k, _v, _s, t)| t).unwrap();
    let result2 = (result2.0)[1..].parse::<i32>().unwrap() * result2.2 as i32;

    println!("result1: {:?} result2: {:?}", result1, result2);
}

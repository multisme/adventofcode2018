use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::char::from_u32;

fn read_input() -> std::string::String {
    let path = Path::new("../test.txt");
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

fn m_distance(a:(i32, i32), b:(i32,i32)) -> i32{
    let x = (a.0 - b.0).abs();
    let y = (a.1 - b.1).abs();
    (x + y)
}

fn main() {

    let s: String = read_input();

    // Parse data
    let coord = s.lines().map(|x| { let mut sp = x.split(", ");
        (sp.next().unwrap().parse::<i32>().unwrap(),
        sp.next().unwrap().parse::<i32>().unwrap())})
        .collect::<Vec<_>>();

    // Find dimension of the maps
    let max;
    {
        let mut v_order = coord.clone();
        let mut h_order = coord.clone();
        v_order.sort_by(|a, b| (a.1).cmp(&b.1));
        h_order.sort_by(|a, b| (a.0).cmp(&b.0));
        max = ((h_order.last().unwrap()).0, (v_order.last().unwrap()).1);
    }

    // Fill the map
    let len = (max.1 * max.0) as usize;
    let mut map = vec![' '; len ];
    let mut map_sum = vec![0i32; len];
    let mut index: usize = 0;
    for j in 0..max.1{
        for i in 0..max.0{
            let mut res: Vec<_> = coord.iter().enumerate()
                .map(|(k, x)| (m_distance(*x, (i,j)), from_u32(57 + k as u32).unwrap()))
                .collect();
            res.sort();
            map_sum[index + i as usize] = res.iter().fold(0, |acc, x| acc + x.0);
            map[index + i as usize] = 
                if res[0].0 != res[1].0{
                    res[0].1
                } else {
                    '.'
                };
        }
        index += max.0 as usize;
    }

    // Create filter to remove infinites areas;
    let mut filter = Vec::new();
    filter.extend(map[0..(max.0 as usize)].iter().cloned());
    filter.extend(map[(len - max.0 as usize)..len].iter().cloned());
    for x in map.iter().step_by(max.0 as usize){ filter.push(*x); }
    for x in map.iter().step_by(max.0 as usize + 1){ filter.push(*x); }

    let mut result1: Vec<(usize, char)> = Vec::new();

    for x in 0..coord.len(){
        let _char = from_u32(57 + x as u32).unwrap();
        if filter.contains(&_char){
            continue ;
        }
        let _count = map.iter().filter(|&x| *x == _char).count();
        result1.push((_count, _char));
    }
    result1.sort();
    let result2 = map_sum.iter().filter(|&x| *x < 10000).count();
    println!("{:?} {:?}", result1.last().unwrap(), result2);
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::char::from_u32;
use std::collections::HashMap;


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

fn find_road() -> (){

}

fn main() {

    let s: String = read_input();

    // Parse data
    let mut _map: HashMap<char, Vec<char>> = HashMap::new();
    let mut _rev_map: HashMap<char, Vec<char>> = HashMap::new();
    let mut _starts: Vec<char> = Vec::new();

    for s in s.lines(){
        let _splits: Vec<_> = s.split_whitespace().collect();
        // Get the key of the algorithm
        let _char_k = _splits[1].parse::<char>().unwrap();
        // Get the value of the algorithm
        let _char_v = _splits[7].parse::<char>().unwrap();
        // Create an entry in the Hashmap - Get that entry back
        let mut p = _map.entry(_char_k).or_insert(Vec::new());
        p.push(_char_v);
        let mut p = _map.entry(_char_v).or_insert(Vec::new());
        p.push(_char_k);
        // Push value in the vector
        _starts.push(_char_v);
        //    }
    }
    
}
*/

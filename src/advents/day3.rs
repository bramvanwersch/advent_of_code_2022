use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};


pub fn day3(nr: i32){
    match nr {
        1=>challenge1(),
        2=>challenge2(),
        _=>println!("Please provide 1 or 2 as challenge number")
    }
}

fn challenge1(){
    let file = File::open("input_files/3_1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut mapping:HashMap<char, i32> = HashMap::new();
    let mut count = 1;
    for i in 97..123{
        mapping.insert(char::from_u32(i).unwrap(), count);
        count += 1;
    }
    for i in 65..91{
        mapping.insert(char::from_u32(i).unwrap(), count);
        count += 1;
    }
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut encountered_chars:HashSet<char> = HashSet::new();
        for (index, c) in line.chars().enumerate(){
            if index < line.len() / 2{
                encountered_chars.insert(c);
            }
            else if encountered_chars.contains(&c){
                total += mapping.get(&c).unwrap();
                break;
            }
        }
    }
    println!("{}", total);
}

fn challenge2(){
    let file = File::open("input_files/3_1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut mapping:HashMap<char, i32> = HashMap::new();
    let mut count = 1;
    for i in 97..123{
        mapping.insert(char::from_u32(i).unwrap(), count);
        count += 1;
    }
    for i in 65..91{
        mapping.insert(char::from_u32(i).unwrap(), count);
        count += 1;
    }
    let mut total = 0;
    let mut inspection_lines:[&str; 3] = ["", "", ""];
    let mut common = HashSet::new();
    for (lindex, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if lindex % 3 == 0 && lindex != 0{

        }
        inspection_lines[lindex % 3] = &line;
    }
}

fn get_common_letter(&lines: [&str; 3]){
    for char in lines[0].chars(){

    }

}

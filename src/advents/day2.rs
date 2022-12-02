use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


pub fn day2(nr: i32){
    match nr {
        1=>challenge1(),
        2=>challenge2(),
        _=>println!("Please provide 1 or 2 as challenge number")
    }
}


fn challenge1(){
    let file = File::open("input_files/2_1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut mapping:HashMap<&str, i32> = HashMap::new();
    mapping.insert("A", 1);
    mapping.insert("B", 2);
    mapping.insert("C", 3);
    mapping.insert("X", 1);
    mapping.insert("Y", 2);
    mapping.insert("Z", 3);

    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(" ").collect();
        let move1 = *mapping.get(moves[0]).unwrap();
        let move2 = *mapping.get(moves[1]).unwrap();
        if move1 == 1 && move2 == 3{
        }
        else if move2 == 1 && move1 == 3{
            score += 6;
        }
        else if move1 == move2{
            score += 3;
        }
        else if move1 < move2{
            score += 6;
        }
        score += move2;
    }
    println!("{}", score);
}

fn challenge2(){
    let file = File::open("input_files/2_1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut mapping:HashMap<&str, i32> = HashMap::new();
    mapping.insert("A", 1);
    mapping.insert("B", 2);
    mapping.insert("C", 3);
    mapping.insert("X", 0);
    mapping.insert("Y", 3);
    mapping.insert("Z", 6);

    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(" ").collect();
        let move1 = *mapping.get(moves[0]).unwrap();
        let move2 = *mapping.get(moves[1]).unwrap();
        let mut second = 0;
        if move2 == 3{
            second = move1
        }
        else if move2 == 6{
            second = move1 + 1;
            if second > 3{
                second -= 3;
            }
        }
        else{
            second = move1 - 1;
            if second < 1{
                second += 3;
            }
        }
        score += second + move2;
    }
    println!("{}", score);
}

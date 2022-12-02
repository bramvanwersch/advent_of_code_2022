use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn day1(nr: i32){
    match nr {
        1=>challenge1(),
        2=>challenge2(),
        _=>println!("Please provide 1 or 2 as challenge number")
    }
}

fn challenge1(){
    let file = File::open("input_files/1_1.txt").unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut total = 0;
    let mut max: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        if line == ""{
            if total > max{
                max = total;
            }
            total = 0;
            continue;
        }
        let nr: i32 = line.parse().unwrap();
        total += nr;
    }
    if total > max{
        max = total;
    }
    println!("{}", max);
}


fn challenge2(){
    let file = File::open("input_files/1_1.txt").unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut total = 0;
    let mut max: [i32; 3] = [0, 0, 0];
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        if line == ""{
            for i in 0..3{
                if total > max[i]{
                    let temp = max[i];
                    max[i] = total;
                    total = temp;
                }
            }
            total = 0;
            continue;
        }
        let nr: i32 = line.parse().unwrap();
        total += nr;
    }

    for i in 0..3{
        if total > max[i]{
            let temp = max[i];
            max[i] = total;
            total = temp;
        }
    }

    println!("{} {} {}", max[0], max[1], max[2]);
    println!("{}", max[0] + max[1] + max[2]);
}
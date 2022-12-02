use std::env;

mod advents;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide 2 arguments. First argument being the day and second one the challenge nr of that day.");
        std::process::exit(-1);
    }
    else{
        let nr_1: i32 = args[1].parse().unwrap();
        let nr_2: i32 = args[2].parse().unwrap();

        match nr_1 {
            1=>advents::main(nr_2),
            _=>()
        }
    }

}

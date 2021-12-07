use std::env;
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let args: Vec<String> = env::args().collect();

    println!("Searching File {}", filename);
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    
    let v: Vec<&str> = contents.split("\n").collect();
    let mut total_changes = 0;
    
    for (idx, val) in v.iter().enumerate() {

        // get the difference between this value in the previous value
        if idx == 0 {
            continue;
        };
        let diff_val: i32 = v[idx -1 ].parse::<i32>().unwrap() - v[idx].parse::<i32>().unwrap();
        if diff_val < 0 {
            total_changes = total_changes + 1;
        };
    };

    println!("Total Changes: {}", total_changes);
}
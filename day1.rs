use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::time::SystemTime;

struct ListOfNumbers {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_file(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut list = ListOfNumbers {
        left: vec![0; 0],
        right: vec![0; 0],
    };

    for line in reader.lines() {
        let li  = line.unwrap();
        
        let l: i32 = (li[0..5]).parse::<i32>().unwrap();
        let r: i32 = (li[8..13]).parse::<i32>().unwrap();

        list.left.push(l);
        list.right.push(r);        
    }

    list.left.sort();
    list.right.sort();

    let mut i = 0;
    let mut diff = 0;

    for l in list.left.iter() {
        print!("Left= {}", list.left.get(i).unwrap());
        println!(" Right= {}", list.right.get(i).unwrap());

        diff += (l - list.right.get(i).unwrap()).abs();    
        i += 1;    
    }

    println!("Diff= {}", diff);

    Ok(())
}

fn main() {
    let file_name = "input.full.txt";

    // Print text to the console.
    println!("Hello Rust World!");

    let before = SystemTime::now();

    let _ = read_file(file_name);

    let after = SystemTime::now();

    let duration = after.duration_since(before)
        .expect("Clock may have gone backwards");
    println!("Duration: {duration:?}");
}
// use std::env;
use std::io::prelude::*;
use std::fs::File;

pub mod general;

pub fn initiate() {
    let mut file = File::open("data/start.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    println!("{}", contents);
    general::start();
}

pub mod asteroid_strike {
    pub fn initiate() {
        println!("WHAMO!");
    }

}

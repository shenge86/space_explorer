// use std::env;
use std::io::prelude::*;
use std::fs::File;

pub mod general;

pub fn initiate() {
    let mut file = File::open("data/start.txt").expect("As a Chief Engineer of the starship to a nearby planet, I needed to provide advice on what to purchase with our initial $1,000,000.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("FILE CANNOT BE OPENED.");
    println!("{}", contents);
    general::purchase();
}

pub mod asteroid_strike {
    pub fn initiate() {
        println!("WHAMO!");
    }

}
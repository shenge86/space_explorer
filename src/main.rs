// external libraries
use std::io;
use rand::Rng;

// internal libraries
// extern crate events
// Help links:
// https://doc.rust-lang.org/stable/rust-by-example/mod/split.html
mod events;
mod spaceship;

mod front_of_house;
use front_of_house::hosting::add_to_waitlist as addw;


// ===========MAIN GAME================ //
fn main() {
    const max_speed: u32 = 300_000; // max spaceship velocity cannot be over speed of light in m/s
    const min_person: u32 = 100; // min number of people below which you lose
    //static MAX_SPEED: i32 = 300_000;
    //static MIN_PERSON: i32 = 100;

    // define global variables
    let mut num_person = 5000;
    let mut speed = 299_001;

    // ready for game
    println!("SPACE EXPLORERS");
    println!("An interstellar exploration game by Shen Ge Creative");
    println!("=================================");
    events::initiate();
    let engine = events::qa1();
    events::trade(engine);

    
    /*
    // print stats
    space_explorer::print_const(max_speed, min_person);
    space_explorer::print_stats(num_person,speed);

    // reset variables
    num_person += 300;
    speed += 5000;
    // speed = reset_speed(speed, max_speed);
    speed = space_explorer::reset_speed(speed, min_person);

    // reprint stats
    space_explorer::print_stats(num_person,speed);
    */
//    events::general::start();
//    events::asteroid_strike::initiate();
//    println!("{}",sc.armor);
}

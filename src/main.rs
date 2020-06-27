// external libraries
use std::io;
use rand::Rng;

// internal libraries
// extern crate events
// HELP:
// https://doc.rust-lang.org/stable/rust-by-example/mod/split.html
mod events;
mod spaceship;

mod front_of_house;
use front_of_house::hosting::add_to_waitlist as addw;

fn main() {
    const max_speed: u32 = 300_000; // max spaceship velocity cannot be over speed of light in m/s
    const min_person: u32 = 100; // min number of people below which you lose
    //static MAX_SPEED: i32 = 300_000;
    //static MIN_PERSON: i32 = 100;

    // define global variables
    let mut num_person = 5000;
    let mut speed = 299_001;

    // ready for game
    println!("SPACE EXPLORER");
    println!("An interstellar exploration game by Shen Ge Creative");
    println!("=================================");
    events::initiate();
//    events::general::start();
//    events::asteroid_strike::initiate();
//    space_explorer::question();
//     spaceship::test();
//    let sc = space_explorer::Spaceship::create_default();
//    let sc = space_explorer::Spaceship {
//        engine_type:String::from("Fusion"),
//        armor: 100,
//        shield: 100,
//        };
    let sc = space_explorer::Spaceship::build_spaceship(String::from("Fusion"),100,100);
    println!("{}",sc.armor);
    
    // print stats
    print_const(max_speed, min_person);
    print_stats(num_person,speed);

    // reset variables
    num_person += 300;
    speed += 5000;
    // speed = reset_speed(speed, max_speed);
    speed = reset_speed(speed, min_person);

    // reprint stats
    print_stats(num_person,speed);
}



// SETTING VALUE FUNCTIONS
fn reset_speed(mut speed: u32, max_speed: u32) -> u32 {
    if speed > max_speed {
        speed = max_speed-1;
    }
    return speed
    // println!("Speed cannot be greater than light speed. Resetting! Speed: {} km/s",speed)
//    return speed;
}



// PRINTING FUNCTIONS
fn print_const(max_speed: u32, min_person: u32) {
    println!("MAX SPEED: {}", max_speed);
    println!("MINIMUM PEOPLE BEFORE LOSING GAME: {}", min_person);
}


fn print_stats(num_person: i32, speed: u32) {
    println!("Number of crewmembers: {}", num_person);
    println!("Spacecraft speed: {}", speed);
}

fn ready_to_eat() {
    addw();
    space_explorer::eat();
}
    

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

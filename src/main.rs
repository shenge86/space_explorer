// external libraries
use std::io;
use rand::Rng;

// internal libraries
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
    println!("Before the launch, you decide to eat first.");
    ready_to_eat();
    
    println!("Would you like to read the instructions? At any time, you can type instruct to see this again.");
    
    println!("Ready to start? (Y/N)");
    let mut input = String::new();

    let mut len = 0;
    while input != "Y" {
        io::stdin().read_line(&mut input).expect("Please try another input.");
   //      let len = input.len();
        len = input.len();
        input.truncate(len-1);
//        input.chars().last().unwrap();
//        trim_newline(&input);
        ready(&input);
    }

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

// I/O FUNCTIONS


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

fn ready(input: &std::string::String) {
    println!("{}",input);
    if input == "Y" {
        println!("Starting!");
        for number in (1..10).rev() {
            println!("{}",number);
        }
        println!("LIFTOFF! Your spacecraft takes off slowly but quickly reaches its maximum speed. Spaceflight is routine these days and this definitely isn't your first rodeo. Yet, every time you're still brimming with excitement like your first time up as a 9-year-old when the launch actually occurs. This time is SPECIAL since you're leaving the solar system! You emit an especially loud squeal much to the embarassment of your first mate (yes, he glanced at you with a rather annoyed expression). In less than a minute, you and your crew are weightless. After all this time, humanity still has not figured out artificial gravity much to everyone's surprise. Well, true artificial gravity. A spinning space station doesn't count. You are glad that you're wearing your seat belt and that everyone else appears to be as well. All the essential items have been bolted down though you do see a few drops of kool-aid floating around. Someone will be reprimanded for that oversight, you thought.");
    }
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

use std::io;
// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
// load up contents of module front_of_house from a file with same name as module, i.e. front_of_house.rs
mod front_of_house;

// get the module called hosting inside the file called front_of_house.rs
pub use crate::front_of_house::hosting;


// SPACESHIP CLASS
pub struct Spaceship {
    pub engine: Engine,
    pub armor: u32,
    pub shield: u32
}


impl Spaceship {
    // constructor
    pub fn build_spaceship(engine: Engine, armor: u32, shield: u32) -> Spaceship {
        Spaceship {
            engine,
            armor,
            shield,
        }
    }
}

// a spaceship has an engine
pub struct Engine {
    pub id: u32,
    pub name: String,
    pub cost: u32
}

impl Engine {
    // constructor
    pub fn build_engine(id: u32, name: String, cost: u32) -> Engine {
        Engine {
            id,
            name,
            cost,
        }
    }

    // constructor 2
    pub fn build_engine_id(id:u32) -> Engine {
        Engine {
            id,
            name = 'BLAH',
            cost = 1000
        }
    }

    pub fn printstats(&self) {
        println!("ID: {}", self.id)
        println!("Name: {}", self.name);
        println!("Cost: {}", self.cost);
    }
}

// FUNCTIONS
pub fn eat() {
    println!("I am ready to eat!");
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn question() {
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
}

pub fn ready(input: &std::string::String) {
    println!("{}",input);
    if input == "Y" {
        println!("Starting!");
        for number in (1..10).rev() {
            println!("{}",number);
        }
        println!("LIFTOFF! Your spacecraft takes off slowly but quickly reaches its maximum speed. Spaceflight is routine these days and this definitely isn't your first rodeo. Yet, every time you're still brimming with excitement like your first time up as a 9-year-old when the launch actually occurs. This time is SPECIAL since you're leaving the solar system! You emit an especially loud squeal much to the embarassment of your first mate (yes, he glanced at you with a rather annoyed expression). In less than a minute, you and your crew are weightless. After all this time, humanity still has not figured out artificial gravity much to everyone's surprise. Well, true artificial gravity. A spinning space station doesn't count. You are glad that you're wearing your seat belt and that everyone else appears to be as well. All the essential items have been bolted down though you do see a few drops of kool-aid floating around. Someone will be reprimanded for that oversight, you thought.");
    }
}

// SETTING VALUE FUNCTIONS
pub fn reset_speed(mut speed: u32, max_speed: u32) -> u32 {
    if speed > max_speed {
        speed = max_speed-1;
    }
    return speed
}



// PRINTING FUNCTIONS
pub fn print_const(max_speed: u32, min_person: u32) {
    println!("MAX SPEED: {}", max_speed);
    println!("MINIMUM PEOPLE BEFORE LOSING GAME: {}", min_person);
}


pub fn print_stats(num_person: i32, speed: u32) {
    println!("Number of crewmembers: {}", num_person);
    println!("Spacecraft speed: {}", speed);
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}


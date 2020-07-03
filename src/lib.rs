use std::io;
// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
// load up contents of module front_of_house from a file with same name as module, i.e. front_of_house.rs
mod front_of_house;

// get the module called hosting inside the file called front_of_house.rs
pub use crate::front_of_house::hosting;


// SPACESHIP CLASS
pub struct Spaceship {
    pub engine_type: String,
    pub armor: u32,
    pub shield: u32
}


impl Spaceship {
    pub fn build_spaceship(engine_type: String, armor: u32, shield: u32) -> Spaceship {
        Spaceship {
            engine_type,
            armor,
            shield,
        }
    }
}

// need to make a subclass called Engine of Spaceship




// FUNCTIONS

// declare some functions
pub fn eat() {
    println!("I am ready to eat!");
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


// Input functions
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



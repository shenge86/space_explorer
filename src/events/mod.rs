// use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub mod general;

pub fn initiate() {
    let mut file = File::open("data/start.txt").expect("As a Chief Engineer of the starship to a nearby planet, I needed to provide advice on what to purchase with our initial funds. When the captain inquires, I have to answer.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Story file cannot be read! Proceeding with story. As Chief Engineer, I had to answer my captain's inquiry on the level of ship preparedness.");
    println!("{}", contents);
}

// CONVERSATIONS
pub fn qa1() -> space_explorer::Engine {
    // Initial purchases
    general::answer1();
    general::question2();
    println!("I pondered on what we have bought so far. Earlier at the emporium, there were a bunch of options.\n");
    println!("For the propulsion, we had a few choices: ");
    println!("1. EM Drive\n");
    println!("2. Helical\n");
    println!("3. Ramjet\n");
    println!("4. Fission\n");
    println!("5. Fusion\n");
    println!("6. Antimatter\n");
    println!("I decided to choose: ");

    let _=io::stdout().flush();

    let mut answer = get_input();
    let mut s = answer.to_string();
    s.pop();

    let c: u32;

    match &s[..] {
        "1" | "EM Drive" 
            => {
                s = "EM Drive".to_string();
                c = 75
            },
        "2" | "Helical" 
            => {
                s = "Helical".to_string();
                c = 300
            },
        "3" | "Ramjet" 
            => {
                s = "Ramjet".to_string();
                c = 800
            },
        "4" | "Fission" 
            => {
                s = "Fission".to_string();
                c = 500
            },
        "5" | "Fusion" 
            => {
                s = "Fusion".to_string();
                c = 2000
            },
        "6" | "Antimatter" 
            => {
                s = "Antimatter".to_string();
                c = 2500
            },
         _  => {
                println!("I could not decide and decided to go with the traditional Fission.");
                s = "Fission".to_string();
                c = 500
           },
    }

    println!("At the time I had purchased, I asked the merchant, \"How much is the {} engine and what are its capabilities?\"", s);

    // let engine = space_explorer::Engine::build_engine(1,String::from(s),c);
    let engine = space_explorer::Engine::build_engine_id(id);
    engine
    
    /*
    let mut t0: bool = true;
    t0 = check_int(answer);
    if t0 {
        println!("True");
    } else {
        println!("False");
    }
    */
}

pub fn trade(engine: space_explorer::Engine) {
    println!("The tradeswoman spoke, \"Well, don't you see it on the sign here?\"");
    engine.printstats();
    println!("Then you thought you heard her mutter under her breathe...");
    quotes();
}

pub fn qa2() {
    let x = get_input().trim().parse::<i64>().unwrap();
    if x == 1 {
        println!("Was that really the best choice?");
    }
}

pub fn buy() {
    general::purchase();
}

pub fn quotes() {
    general::quote();
}


// EVENTS
pub mod asteroid_strike {
    pub fn initiate() {
        println!("WHAMO!");
    }

}

// FUNCTIONS FOR INPUTS 
fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Nothing was spoken.");
    buffer
}

// fn check_int(answer: &str) -> bool {
// fn check_int<S: Into<String>>(answer: S) -> bool {
/*
fn check_int<S: AsRef<str>>(input: S) -> bool {
    let answer = input.as_ref();
    let trimmed = answer.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return true,
        Err(..) => return false
    }
}
*/

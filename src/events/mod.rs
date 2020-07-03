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

pub fn qa1() {
    // Initial purchases
    general::answer1();
    general::question2();
    println!("I pondered on what we have bought so far. Earlier at the emporium, there were a bunch of options.\n");
    println!("For the propulsion, we had a few choices (enter #): ");
    println!("1. EM Drive\n");
    println!("2. Helical Engine\n");
    println!("3. Interstellar Ramjet\n");
    println!("4. Beamed Propulsion\n");
    println!("5. Nuclear Pulse\n");
    println!("6. Fission Electric\n");
    println!("7. Fission Fragment\n");
    println!("8. Nuclear Fusion\n");
    println!("9. Antimatter\n");

    let _=io::stdout().flush();

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("I said nothing.");

    print!("> ");

    // match answer.as_str() {
    match &answer[..] {
        "1\n" => println!("Inputted 1"),
        _ => println!("That is not a valid choice."),
    }

    println!("I chose {}", answer);
    
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

pub fn qa2() {
    let x = get_input().trim().parse::<i64>().unwrap();
    if x == 1 {
        println!("Was that really the best choice?");
    }
}

pub fn buy() {
    general::purchase();
}

pub mod asteroid_strike {
    pub fn initiate() {
        println!("WHAMO!");
    }

}

// FUNCTIONS FOR INPUTS 
fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
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

#[allow(dead_code)]
use rand::Rng;

pub fn start() {
    println!("I am anticipating another event to happen real soon...");
}

// questions for the player
pub fn answer1() {
    println!("\"The ship is well-repared to handle everything that will confront us.\"\n");
}

pub fn question2() {
    println!("\"What do we have so far?\"\n");
}

pub fn purchase() {
    println!("This may sound like a lot of money but it really isn't...");
}

pub fn quote() {
    let mut s = vec!["We fill in our blanks in the crossroads of life",];
    s.push("The element of air flies freely inside of you. Impossible to tie down, you prefer to slip and maneuver your way through the obstacles in your path. You're not the most responsible person around, preferring to act on whims and never taking anything too seriously. The world holds as many possibilities as you allow it, and nothing can slow you down for long.");
    s.push("People who are intelligent can't not think.");
    s.push("I think that's a mark of intelligence.");
    s.push("Before love comes respect for who that person is and so it's unlikely I will fall in love with a sociopath.");
    s.push("It's a never-ending queue!");
    s.push("We've been talking for 5 hours?! What the hell is wrong with you?");
    s.push("No rest means no rust.");

//    println!("{}",&s.len());

    // Get a random quote
    let si = rand::thread_rng().gen_range(0,s.len());
    println!("{}",&s[si]);
}

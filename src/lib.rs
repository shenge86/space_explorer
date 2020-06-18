// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
// load up contents of module front_of_house from a file with same name as module, i.e.
// front_of_house.rs
mod front_of_house;

// get the module called hosting inside the file called front_of_house.rs
pub use crate::front_of_house::hosting;

// declare some functions
pub fn eat() {
    println!("You are ready to eat!");
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}



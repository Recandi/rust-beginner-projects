extern crate rust_beginner_projects;
use rust_beginner_projects::fibonacci;

fn main() {
    for i in 1..11{
        println!("{}", fibonacci::fib(i));
    }
}

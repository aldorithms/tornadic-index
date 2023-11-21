use crate::composites::stp::stp;

mod composites;

fn main() {
    println!("Hello, world!");
    let SigTor = stp(3000.0, 1000.0, 600.0, 30.0, -50.0 );
    println!("The Significant Tornado Parameter is: {:.2}",  SigTor);
}
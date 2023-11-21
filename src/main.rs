mod composites;

fn main() {
    println!("Hello, world!");
    println!("STP: {}", composites::stpc(4000.0, 0.0, 450.0, 30.0, -50.0));
    println!("VTP: {}", composites::vtp(4000.0, 0.0, 450.0, 3.0, -5.0, 100.0, 13.0));
}
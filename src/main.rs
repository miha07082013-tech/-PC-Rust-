use std::io;
fn main() {
    println!("Okay?");
    println!("===Pipu?===");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Error");
    loop {
        println!("{}", a);
    }
}

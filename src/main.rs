use std::io;
fn main() {
    println!("===Pipu?===");
    let mut a = String::new();
    io::stdn().read_line(&mut a).expect("Error");
    loop {
        println!("{}", a);
    }
}

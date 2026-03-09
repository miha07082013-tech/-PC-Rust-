use std::io;
fn main() {
    println!("===Pipu?===");
    let mut a = String::new();
    io::stidn().read_line(&mut a).expect("Error");
    loop {
        println!("{}", a);
    }
}

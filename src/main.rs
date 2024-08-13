use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome on guess the number !");
    let secret_number = rand::thread_rng().gen_range(0..101);
    loop {
        println!("Enter a number between 0 to 100");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("error");
        let number: u32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        match number.cmp(&secret_number) {
            Ordering::Less => println!("more"),
            Ordering::Greater => println!("less"),
            Ordering::Equal => {
                println!("Succesful the number was {}", secret_number);
                break;
            }
        }
    }
}

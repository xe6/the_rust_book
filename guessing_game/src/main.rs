// std - standard librabry alias
// If not imported with use - you can use lib methods as std::io::read_line();
use std::io; // Import
use std::cmp::Ordering;
use rand::Rng; // Rng - is a trait

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // Create mutable variable 'guess' and assign new String object to it
        let mut guess = String::new();
    
        /*
            The & indicates that this argument is a reference, which gives you a way 
            to let multiple parts of your code access one piece of data
            without needing to copy that data into memory multiple times.
    
            If you will need to change the object by its reference - pass &mut attribute!
        */
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        /*
            io::stdin().read_line() returns io::Result
            For Result, the variants are Ok or Err.
            The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value.
            The Err variant means the operation failed, and Err contains information about how or why the operation failed.
        */
    
        // Convert to u32.
        // Rust allows us to shadow the previous value of guess with a new one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
    
        // The cmp method compares two values and can be called on anything that can be compared.
        // It takes a reference to whatever you want to compare with.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("Input: {}", guess);
    }



}

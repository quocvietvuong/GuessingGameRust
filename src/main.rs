use std::io; // if you want to use types or functions, you have to bring that into the program scope
use rand::Rng; // Rng is a trait???
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101); // take a range expression as an argument
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new(); // in Rust, variables are immutable by default, therefore you have to add mut before any variable declaration to 
        // be able to change the value of it during program.
        // new() is a function (static method of string) that returns a new instance of a string
        // :: operator indicates that new() is a static method of String
        
        io::stdin() // stdin() function returns an instance of std::io::Stdin
            .read_line(&mut guess) // call the read_line method on the instance Stdin. read_line function appends the passed contents, not overwrite. & indicates reference
            .expect("Fail to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // trim(): eliminate white spaces, and \n character, returns only the number
        // parse(): parses a string into some kind of number
        // inclusive in the lower bound but not with the upper bound
        
        // thread_rng: function to generate random number, local to the current thread, seeded by the operating system
        // gen_range: function to be called on the random generator
        
        
        // read_line returns io::Result {Ok, Err}. An instance of io::Result has a 'expect' method that you can call
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
    
}

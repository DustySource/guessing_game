use std::io;
use rand::Rng;


fn main() {
    println!("Henlo Fren!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100);
   


 
    loop{
        
         // println!("The secret number is: {}", secret_number);
        println!("Please Guess a Number!");
        let mut guess = String::new();
        
        
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    
       
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
     

    match guess.cmp(&secret_number){
        std::cmp::Ordering::Less => println!("Too Small!! :O"),
        std::cmp::Ordering::Equal => {
            println!("You Guessed It!!!");
            secret_number = rand::thread_rng().gen_range(1..=100);
         
        }
        std::cmp::Ordering::Greater=> println!("TOO BIG D:"),
        

    }
}

}

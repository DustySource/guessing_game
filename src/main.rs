use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Henlo Fren!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);


 
    loop{

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
        std::cmp::Ordering::Less => println!("Too Small! shrimp x3"),
        std::cmp::Ordering::Equal => {
            println!("Nyaa!");
            break;
        }
        std::cmp::Ordering::Greater=> println!("Its TOO BIG NYAAAAA~ D:"),
        

    }
}

}

use std::io;
use rand::Rng;

fn read_input() -> i32 {
    let mut input : String = String::new();
    let _ = io::stdin().read_line(&mut input);
    let mut res = -1;
    match input.trim().parse::<i32>() {
        Ok(n) => {
            println!("You choose number: {}", n);
            res = n;},
        Err(e) => {
            println!("Error: {}", e);
            println!("Please put a good number");
        },
    }
    
    return res;
}

fn main() {
    let mut rng = rand::rng();
    let result : i32 = rng.random_range(0..1001);
    let mut value : i32 = -1;
    let mut turn : i32 = 0;
    while value != result
    {
        println!("Try to find the random number chosen between 0 and 1000 : ");
        turn += 1;
        value = read_input();
        if value > 1000 || value < 0
        {
            println!("You have to find a value between 0 and 1000");
        }
        else if value < result
        {
            println!("The value to find is higher");
        }
        else if value > result
        {
            println!("The value to find is lower");
        }
    }
    println!("GG! You've found the result in {turn} turns!");
}



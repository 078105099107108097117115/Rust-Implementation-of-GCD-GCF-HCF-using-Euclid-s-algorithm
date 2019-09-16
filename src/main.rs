#![allow(unused)]
use std::io::{self,Write};

fn main() {
    //println!("Hello, world!");
    println!("Enter the numbers whose greatest common divisor(GCD/GCF) you'd like to calculate.");
    
    let mut first_input = String::new();
    let mut second_input = String::new();
    loop
    {
        print!("First number :");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut first_input){
            Ok(n) => { 
                print!("{} bytes read.",n);
                io::stdout().flush().unwrap();
                println!("  Your input ;->{}",first_input);
            }
            Err(error) => println!("error => {}",error),
        }
        let first_input : u64 = match first_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };    
        print!("Enter second no:");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut second_input){
            Ok(n) => {
                print!("{} bytes read.",n);
                io::stdout().flush().unwrap();
                println!("  Your input = {}",second_input);
            }
            Err(error) => println!("error => {}",error),

        }
        let second_input : u64 = match second_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        let output = gcd(first_input,second_input);
        println!("{}",output);
        break;
    }
}



//<********************************************************>//
//The gcd function implements Euclid's algorithm as follows:
//<********************************************************>//

fn gcd(n : u64,m : u64) -> u64{
    let mut dividend : u64;
    let mut divisor : u64;
    if n > m {
        dividend = n;
        divisor = m;
    }else {
        dividend = m;
        divisor = n;
    }
    let mut modulo : u64 = dividend % divisor;
    loop {
        if modulo == 0 {
            break;
        }else{
            dividend = divisor;
            divisor = modulo;
            modulo = dividend % divisor;
        }
    }
    divisor
}







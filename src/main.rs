extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    //create empty struct to store players info
    struct Player {
        name: String,
        pot: i32,
	};

    // set variables
    let dice1 = rand::thread_rng().gen_range(1, 7);
    let dice2 = rand::thread_rng().gen_range(1, 7);
    let mut roll = dice1 + dice2;

    println!("The total is: {}", roll);
    // create an instance 
    let mut nick = Player {
        name: String::from("Nick"),
        pot: 100,
	};

    // get user input 
    println!("Place pass or no pass bet(pass/no pass): ");

    // get choice - make mutable since read_line requires mut data
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");
    
    
    println!("A {} and {} were rolled. ", dice1, dice2);
    
    if choice.trim() == "pass" {
        println!("Test works");
        if roll == 7 || roll == 11 {
            println!("Second test successful, equaled pass");
        }
        else {
            println!("Executing normally, didn't equal above condition for pass");
        }
    }
    else if choice.trim() == "no pass" || choice.trim() == "nopass" {
        if roll == 2 || roll == 3 || roll == 11 {
            println!("Second test successful, equaled no pass");
        }
        else {
            println!("Executing normally, didn't equal above condition for no pass");
        }
    }


    //end of main function
}

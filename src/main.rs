/*
Possibly make a global roll, 
this will persist for each round. 

pass arguments into both point round and pass round
depending on number, point or pass will be executed.
*/

extern crate rand;

use std::io;
use rand::Rng;

struct Player {
    name: String,
    pot: i32,
}


fn main() {

    let mut roll: i32 = dice_roll();
    pass_round(roll);
    
    
}

fn dice_roll() -> i32 {
    let dice1 = rand::thread_rng().gen_range(1, 7);
    let dice2 = rand::thread_rng().gen_range(1, 7);
    let mut roll = dice1 + dice2;
    println!("A {} and {} were rolled. ", dice1, dice2);

    return roll;
}

// Separate the pass round into smaller modules.  
fn pass_round(mut comeout_roll: i32) -> i32 {

    // call dice_roll function
    println!("Roll is: {}", comeout_roll);

    // Create an instance of Player.
    let mut nick = Player {
        name: String::from("Nick"),
        pot: 100,
	}; 

    // Get user input. 
    println!("Place pass or no pass bet(pass/no pass): ");

    // Get choice - make mutable since read_line requires mut data
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");
    
    
    // Issue with choice resolved. Needed trim method to compare to other strings. 
    if choice.trim() == "pass" {
        if comeout_roll == 7 || comeout_roll == 11 {
            println!("Pass bets won, crap out bets lose.");
        }
        else {
            println!("Executing normally, move to point round");
            // point_round();
        }
    }
    else if choice.trim() == "no pass" || choice.trim() == "nopass" {
        if comeout_roll == 2 || comeout_roll == 3 || comeout_roll == 12 {
            println!("Crap out bets won, pass bets lose.");
        }
        else {
            println!("Executing normally, move to point round");
            // point_round();
        }
    }

    return comeout_roll;

}


// fn point_round() {
//     let mut return_point = pass_round();
//     println!("Point num persists in point round: {}", return_point);

// }

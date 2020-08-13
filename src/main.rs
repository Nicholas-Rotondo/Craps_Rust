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

    // set variables uptop 
    let dice1 = rand::thread_rng().gen_range(1, 7);
    let dice2 = rand::thread_rng().gen_range(1, 7);
    let pass_bet = 10;
    let mut rolling = true;
    let pass_combo = vec![7, 11];
    let no_pass_combo = vec![2, 3, 12];

    // create an instance 
    let mut nick = Player {
        name: String::from("Nick"),
        pot: 100,
	};

    // get user input 
    println!("Place pass or no pass bet(pass/no pass): ");

    // get choice - make mutable since read_line requires mut data
    let mut choice.trim() = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");





    println!("A {} and {} were rolled. ", dice1, dice2);
    if (dice1 + dice2 == 7) || (dice1 + dice2 == 11) { 
        if choice.trim() == "pass" {
            println!("Pass wins");
        }
    }
    else if (dice1 + dice2 == 2 )|| (dice1 + dice2 == 3) || (dice1 + dice2 == 12) {
        if choice.trim() == "no pass" {
            println!("Craps wins.");
		  }
		}
    else {
            println!("Point set.");
		}



    //if player placed on pass double his money. If he place on no pass lose his bet. 
    // same goes for if they placed on no pass. Double or deduct bet.
}

// import section
extern crate rand;
extern crate simple_excel_writer as excel;

// external dependencies 
use excel::*;
use std::io;
use rand::Rng;

// import within directory
use crate::player::create_player::Player;

// mod section
mod player;


fn main() {

    let roll: i32 = dice_roll();
    let mut player1 = add_player();
    pass_round(roll, player1);  
    
}

fn add_player() -> Player {
    let mut player1 = Player::new_player();
    return player1;
}

fn dice_roll() -> i32 {
    let dice1 = rand::thread_rng().gen_range(1, 7);
    let dice2 = rand::thread_rng().gen_range(1, 7);
    let roll = dice1 + dice2;

    return roll;
}

fn user_input() -> i32 {
    println!("Place a bet on point or another number: ");

    // Get choice - make mutable since read_line requires mut data
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");
    let mut choice: i32 = choice.trim().parse().expect("Please type a number!");

    return choice;
}

// Separate the pass round into smaller modules.  
fn pass_round(comeout_roll: i32, mut player1: Player) {

    // call dice_roll function
    println!("Roll is: {}", comeout_roll);

    // Get user input. 
    println!("Place pass or no pass bet(pass/no pass): ");

    // Get choice - make mutable since read_line requires mut data
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");
    
    
    // Issue with choice resolved. Needed trim method to compare to other strings. 
    // add restart function when [2, 3, 7, 11, 12]
    if comeout_roll == 7 || comeout_roll == 11 {
        if choice.trim() == "pass" {
            player1.won();
            println!("Pass bets won, crap out bets lose.");
        }
        else {
            player1.lost();
            println!("Lost bet.");
        }
    }
    else if comeout_roll == 2 || comeout_roll == 3 || comeout_roll == 12 {
        if choice.trim() == "no pass" || choice.trim() == "nopass" {
            player1.won();
            println!("Crap out bets won, pass bets lose."); 
        }
        else {
            player1.lost();
            println!("Lost bet.");
        }
    }
    else {
        println!("Point is now {}", comeout_roll);
        point_round(comeout_roll, player1);
    }

}

fn point_round(point: i32, mut player1: Player){

    // copy point and keep rolling until a 7 is rolled. if a 7 is rolled break
    // can't compare i32 and &i32. two different types.
    let ref_point = point;

    //must be mutable
    let mut flag = false;

    let roll: i32 = dice_roll();
    
    while flag == false {

        // let roll: i32 = dice_roll(); 
        player1.update_bet();
        println!("Roll is: {}", roll);

        let mut choice = user_input();

        if choice == ref_point {
            player1.won();
        } else if roll == 7 {
            println!("Crapout, session over.");
            player1.lost();
            restart(flag);
            flag = true;
        }
    }

}

fn restart(mut flag: bool) -> bool {

    let roll: i32 = dice_roll();

    println!("Would you like to play again(y/n): ");

    // Get choice - make mutable since read_line requires mut data
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");

    flag = false;
    if choice.trim() == "y" || choice.trim() == "yes" {
        main();
    } else if choice.trim() == "n" || choice == "no" {
        println!("Take care.");
        flag = true;
    }

    return flag;
}

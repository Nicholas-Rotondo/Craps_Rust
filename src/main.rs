// import section
extern crate rand;
use std::io;
use rand::Rng;

// import within directory
use crate::player::create_player::Player;

// mod section
mod player;


fn main() {

    let roll: i32 = dice_roll();
    pass_round(roll);  
    
}

fn dice_roll() -> i32 {
    let dice1 = rand::thread_rng().gen_range(1, 7);
    let dice2 = rand::thread_rng().gen_range(1, 7);
    let roll = dice1 + dice2;

    return roll;
}

// Separate the pass round into smaller modules.  
fn pass_round(comeout_roll: i32) {

    // call dice_roll function
    println!("Roll is: {}", comeout_roll);

    // Create an instance of Player.
    let nick = Player {
        name: String::from("Nick"),
        pot: 100,
    }; 

    
    println!("{}: ", nick.get_pot());
    // println!("{}, {} show it has initialized and can be accessed: ", nick.name, nick.pot);

    // Get user input. 
    println!("Place pass or no pass bet(pass/no pass): ");

    // Get choice - make mutable since read_line requires mut data
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    .expect("Failed to read line");
    
    
    // Issue with choice resolved. Needed trim method to compare to other strings. 
    if comeout_roll == 7 || comeout_roll == 11 {
        if choice.trim() == "pass" {
            println!("Pass bets won, crap out bets lose.");
        }
        else {
            println!("Executing normally, move to point round");
        }
    }
    else if comeout_roll == 2 || comeout_roll == 3 || comeout_roll == 12 {
        if choice.trim() == "no pass" || choice.trim() == "nopass" {

            println!("Crap out bets won, pass bets lose.");
        }
        else {
            println!("Executing normally, move to point round");
        }
    }
    else {
        println!("Point is now {}", comeout_roll);
        point_round(comeout_roll);
    }

}

fn point_round(point: i32){

    // copy point and keep rolling until a 7 is rolled. if a 7 is rolled break
    // can't compare i32 and &i32. two different types.
    let ref_point = point;

    //must be mutable
    let mut flag = false;
    
    while flag == false {
        let roll: i32 = dice_roll(); 
        println!("Roll is: {}", roll);

        if roll == ref_point {
            point_round(ref_point);
            flag = true;
        } else if roll == 7 {
            println!("Crapout, session over.");
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
        pass_round(roll);
    } else if choice.trim() == "n" || choice == "no" {
        println!("Take care.");
        flag = true;
    }

    return flag;
}

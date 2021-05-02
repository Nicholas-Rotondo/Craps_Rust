// import section
extern crate rand;
extern crate simple_excel_writer as excel;
use std::any::type_name;

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

    let mut players_vector: Vec<Player> = add_players();

    pass_round(roll, players_vector);  
    
}


fn add_players() -> Vec<Player> {
    let mut player_vector: Vec<Player> = Vec::new();
    let mut count: i32 = 0;

    println!("Welcome to Craps! How many players are joining: ");

    let mut player_count = String::new();
    io::stdin()
    .read_line(&mut player_count)
    .expect("Failed to read line");

    let player_count: i32 = player_count.trim().parse().expect("How many players are participating: ");

    while count < player_count {
        let mut player = Player::new_player();
        player_vector.push(player);
        count += 1; 
    }

    // print players names to show current users at the "table"
    println!("{:?}", player_vector);
    return player_vector;

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
    let mut choice: i32 = choice.trim().parse().expect("Please type a number: ");

    return choice;
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn pass_round(comeout_roll: i32, players_vector: Vec<Player>) {

    // call dice_roll function
    println!("Roll is: {}", comeout_roll);
    let mut check = false;

    let mut copy_players_vector = players_vector.clone();

    for mut player in players_vector {
        
        let player_pass = player.get_pass();
        let as_str_pass = player_pass.as_str();

        println!("{}", type_of(as_str_pass));


        // match needs to be passed an &str type variable - not String type variable
        // values don't match. could be a struct(&str - as_str_pass) compared to regular &str   


        if comeout_roll == 7 || comeout_roll == 11 {
            if player.get_pass() == "pass" {
                player.won();
                println!("Pass bets won, {:?} won", player);
            }
            else {
                player.lost();
                println!("{:?} lost the bet.", player);
            }
        }
        else if comeout_roll == 2 || comeout_roll == 3 || comeout_roll == 12 {
            if player.get_pass() == "no pass" || player.get_pass() == "nopass" {
                player.won();
                println!("Crap out bets won. {:?} won", player); 
            }
            else {
                player.lost();
                println!("{:?} lost the bet.", player);
            }
        }
        else {
            println!("{} point round is: ", comeout_roll);
            point_round(comeout_roll, (copy_players_vector).to_vec());
        }
    }
}
    
// BRB POOPING

fn point_round(point: i32, mut players_vector: Vec<Player>){

    // copy point and keep rolling until a 7 is rolled. if a 7 is rolled break
    // can't compare i32 and &i32. two different types.
    let ref_point = point;

    //must be mutable
    let mut flag = false;

    println!("You have made it here successfully.")

    
    // while flag == false {

    //     let roll: i32 = dice_roll(); 
    //     //players_vector.update_bet();
    //     println!("Roll is: {}", roll);
        
    //     //mut choice is trimmed in the player module and parsed to equate to i32(signed ints)
    //     let mut choice = user_input();

    //     if choice == ref_point {
    //         //players_vector.won();
    //     } else if roll == 7 {
    //         println!("Crapout, session over.");
    //         //players_vector.lost();
    //         restart(flag);
    //         flag = true;
    //     }
    // }
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

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

fn pass_round(comeout_roll: i32, players_vector: Vec<Player>) {

    // call dice_roll function
    println!("Roll is: {}", comeout_roll);
    let mut check = false;

    let mut copy_players_vector = players_vector.clone();

    for mut player in players_vector {
        
        let player_pass = player.get_pass();
        let as_str_pass = player_pass.as_str();


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
            println!("Point is {} ", comeout_roll);
            point_round(comeout_roll, (copy_players_vector).to_vec());
        }
    }
}
    

fn point_round(point: i32, mut players_vector: Vec<Player>){

    // copy point and keep rolling until a 7 is rolled. if a 7 is rolled break
    // can't compare i32 and &i32. two different types.
    let ref_point = point;

    //must be mutable
    let mut flag = false;

    println!("You have made it to the point round successfully.");

    for mut player in players_vector {

        let player_pass = player.get_pass();
        let as_str_pass = player_pass.as_str();

        while flag == false {

            //issue arising when two players are involved. refer to word doc
            player.update_pass_bet();
            player.update_bet();

            let roll: i32 = dice_roll(); 

            if roll != 7 {
                // get_pass() returns &str - point is an i32 type
                // change point to &str or 
                let player_pass_num: i32 = player.get_pass().trim().parse().expect("Place string as number");
                println!("Roll is: {}", roll);
                if player_pass_num == point {
                    player.won();
                }
            } else {
                println!("7 has been rolled");
                player.lost();
                flag = true;
            }
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

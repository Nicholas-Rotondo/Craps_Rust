fn test() {
    use crate::player::create_player::Player;

    mod player;

    let roll: i32 = dice_roll();
    let mut player1 = add_player();
    pass_round(roll, player1);  
    
}
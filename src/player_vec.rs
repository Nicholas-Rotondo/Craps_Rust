use crate::player::create_player::Player;

pub struct Players {
    list: Vec<Player>,
}

impl Players {
    fn add_players() -> Vec<Player> {
        let mut player_vector: Vec<Player> = Vec::new();
        let mut count: i32 = 0;
    
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
    
        return player_vector
    
    }
}
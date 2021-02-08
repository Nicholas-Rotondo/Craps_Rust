pub mod create_player {
    pub struct Player {
        pub name: String,
        pub pot: i32,
    }
    
    use std::io;
    impl Player {

        // use self when working with the instance of self.
        // otherwise just an associated function used for returning and constructing a new instance.

        // starting pot for the round. need at least $100 else can't play.
        pub fn set_pot() -> i32 {
            println!("Minimum pot to play is $100: ");

            let mut starting_pot = String::new();
            io::stdin()
            .read_line(&mut starting_pot)
            .expect("Failed to read line");

            // used shadowing technique from guessing game.
            let mut starting_pot: i32 = starting_pot.trim().parse().expect("Please type a number!");
            
            return starting_pot;
        }

        // set bet
        pub fn make_bet(&mut self) -> i32 {
            println!("Minimum amount to bet is $5. Please place a bet: ");

            let mut bet = String::new();
            io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

            let bet: i32 = bet.trim().parse().expect("Please type a number!");
             
            self.pot -= bet;
            println!("Pot: {}", self.pot);
            self.pot
        }

        // set name from command line.
        pub fn set_name() -> String {
            println!("Please enter a name to play: ");

            let mut name = String::new();
            io::stdin().read_line(&mut name)
            .expect("Failed to read line");

            return name;
        }

        // create new players on the fly.
        pub fn new_player() -> Self {
            Player {
                name: Self::set_name(),
                pot: Self::set_pot(),
            }
        }

    }
}
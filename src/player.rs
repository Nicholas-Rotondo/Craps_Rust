pub mod create_player {
    #[derive(Clone)]
    #[derive(Debug)]

    pub struct Player {
        // change from String to str type. 1
        pub name: String,
        // change from String to str type. 2
        pub pass_bet: String,
        pub pot: i32,
        pub bet: i32,
        // possible make another field while holds amount bet.
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
        pub fn set_bet() -> i32 {
            println!("Minimum amount to bet is $5. Please place a bet: ");

            let mut bet = String::new();
            io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

            let bet: i32 = bet.trim().parse().expect("Please type a number!");
             
            return bet;
        }

        // set name from command line.
        // might not need to change from String to str.
        pub fn set_name() -> String {
            println!("Please enter a name to play: ");

            let mut name = String::new();
            io::stdin().read_line(&mut name)
            .expect("Failed to read line");

            return name;
        }

        pub fn won(&mut self) -> &i32 {
            let mut bet = self.bet;
            self.pot += bet;
            println!("You won! Pot is now: {}", self.pot);
            &self.pot
        }

        pub fn lost(&mut self) -> &i32 {
            let mut bet = self.bet;
            self.pot -= bet;
            println!("You lost. Pot is now: {}", self.pot);
            &self.pot
        }


        // change from String to str type. 3
        pub fn set_pass_bet() -> String {
            println!("Please place pass or no pass: ");

            let mut pass_bet = String::new();
            io::stdin().read_line(&mut pass_bet)
            .expect("Failed to read line");
        

            return pass_bet.trim().to_string();
        }

        //getter methods
        pub fn get_pass(&self) -> &String{
            &self.pass_bet
        }
        
        pub fn get_name(&mut self) -> &String {
            &self.name
        }

        //update methods below

        // when returning &=borrow reference &self.pass_bet must return a &String to match the values.
        // change from String to str type. 4
        pub fn update_pass_bet(&mut self) -> &String {
            println!("Place point round bets: ");

            let mut pass_bet = String::new();
            io::stdin().read_line(&mut pass_bet)
            .expect("Failed to read line");

            self.pass_bet = pass_bet.trim().to_string();
            &self.pass_bet
        }

        pub fn update_bet(&mut self) -> &i32 {
            println!("Update bet amount for point round: ");

            let mut bet = String::new();
            io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

            let mut bet: i32 = bet.trim().parse().expect("Please type a number!");
            self.bet = bet;
            self.pot -= bet;
            println!("Bet is {} and pot {}", self.bet, self.pot);
            &self.bet         
        }


        // create new players on the fly.
        pub fn new_player() -> Self {
            Player {
                name: Self::set_name(),
                pass_bet: Self::set_pass_bet(),
                pot: Self::set_pot(),
                bet: Self::set_bet(),
            }
        }

    }
}
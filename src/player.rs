pub mod create_player {
    pub struct Player {
        pub name: String,
        pub pot: i32,
    }

    impl Player {

        // take command line argument to take players names
        // eventually create multiple players in one game instance. 

        // function works but is it logically correct/necessary?
        // if i comment this out my struct will still work in other file.
        // not sure if i need this associated function.

        pub fn new_player(name: String, pot: i32) -> Player {
            Player {
                name: String::from(name),
                pot: i32::from(pot),
            }
        }

        // use self when working with the instance of self.
        // otherwise just an associated function used for returning and constructing a new instance.
        pub fn get_pot(&self) -> i32 {
            self.pot
        }

    }
}
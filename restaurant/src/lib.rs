mod front_house {
    pub mod hosting {
      pub fn add_to_waitlist() {
        super::subtract();
      }
    }
    fn subtract() {

    }
    }

    pub use crate::front_house::hosting; // external code can refer to this module

    pub fn add() {
        hosting::add_to_waitlist();
    }


    mod back_house {
        pub struct breaky {
            drink: String,
            pub food: String
        }

        impl breaky {
            pub fn make(bread: &str) -> breaky {
                breaky {
                    drink: String::from("juice"),
                    food: bread.to_string()
                }
            }
        }
    }

    fn eat() {
        let mut meal = back_house::breaky::make("oat");

        meal.food = String::from("multigrain");
    }
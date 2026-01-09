#[derive(Debug)]
enum State {
    jersey,
    chicago
}
enum comic {
    Marvel(String, u64),
    Dc(String, bool)
}
// enums can be linked with each other
// let marvel = comic::Marvel(State::jersey, 56)
// Option enum for generic types
// match value { Some(T) => {execution}, None => {execution}    }
// cant add u32  Option<u32> need to unwrap first i.e unrwap_or(default)
// match { _=> no_match(value)}


enum Option<T> {
    None,
    Some(T)
}



enum Coin {
    penny,
    nickle,
    dime(State)
}

pub(crate) fn enums() {
    let type1 = comic::Marvel(String::from("spidey"), 45);
    let type2 = comic::Dc(String::from("bat"), true);

    let some_number = Some(5);
    let some_char = Some('e');
    let nonce: std::prelude::v1::Option<i32> = None;

    let m = 5;

    let c = m + some_number.unwrap_or(5);

    let coin = Coin::dime(State::jersey);

    let result = match coin {
        Coin::penny => {
            println!("penny");
            2
        },
        Coin::nickle => 3,
        Coin::dime(state) => {
            println!("{state:?}");
            2
        }
    };

    let z = plusone(Option::Some(5));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player() {}

    let conf = Some(99999);

    if let Some(max) = conf {
       println!("matched");
    }

}

fn plusone(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(i) => Option::Some(i + 1),
        _ => Option::None,
    }
}
#[derive(Debug)]
enum UsState {
    _Alabama,
    _Alaska,
}

enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match 后表达式值可以是任意类型
    match coin {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::_Alabama)));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 不做任何事
        _ => (),
    }
}

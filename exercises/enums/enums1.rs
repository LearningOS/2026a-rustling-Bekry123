// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    Quit,
    Echos,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echos);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
